use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

const CANVAS_WIDTH: u32 = 720;
const CANVAS_HEIGHT: u32 = 720;
const GRID_POINTS: usize = 100;

mod input;
use input::{FnInput, Input, Set};

enum Msg {
    Left(ChangeData),
    Right(ChangeData),
    AddFnInput,
    Function(usize, ChangeData),
    ToggleFunctionFormat(usize),
    Auxiliary(Set),
}
struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    input: Input,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            canvas_ref: NodeRef::default(),
            input: Input::default(),
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        let canvas: HtmlCanvasElement = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(CANVAS_WIDTH);
        canvas.set_height(CANVAS_HEIGHT);

        let backend: CanvasBackend = CanvasBackend::with_canvas_object(canvas).unwrap();

        // Plotters manipulation
        {
            let root = backend.into_drawing_area();
            root.fill(&WHITE).unwrap();

            let grid =
                itertools_num::linspace(self.input.domain.0, self.input.domain.1, GRID_POINTS);

            let mut values_collection = vec![];
            let mut overall_min = f64::INFINITY;
            let mut overall_max = f64::NEG_INFINITY;

            for function_input in &self.input.functions {
                match function_input {
                    FnInput::Analytical { expression, .. } => {
                        let values: Vec<f64> =
                            grid.clone().map(|x| expression.eval(&[x])).collect();
                        let (min, max) = values.iter().minmax().into_option().unwrap();
                        overall_min = min.min(overall_min);
                        overall_max = max.max(overall_max);
                        values_collection.push(values);
                    }
                    FnInput::Points(_) => todo!(),
                }
            }

            let mut chart_builder = ChartBuilder::on(&root);
            if self.input.x_axis {
                chart_builder.set_label_area_size(LabelAreaPosition::Bottom, 40);
            }
            if self.input.y_axis {
                chart_builder.set_label_area_size(LabelAreaPosition::Left, 40);
            }
            if self.input.title {
                chart_builder.caption(self.input.title_string.clone(), ("Arial", 30));
            }

            let delta = overall_max - overall_min;
            let mut chart = chart_builder
                .build_cartesian_2d(
                    self.input.domain.0..self.input.domain.1,
                    (overall_min - delta / 100.)..(overall_max + delta / 100.),
                )
                .unwrap();

            let mut mesh_style = chart.configure_mesh();
            if !self.input.mesh {
                mesh_style.disable_mesh();
            }
            mesh_style.draw().unwrap();

            for values in values_collection {
                chart
                    .draw_series(LineSeries::new(grid.clone().zip(values), &BLACK))
                    .unwrap();
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Left(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change left to {}", x);
                    let proposal: f64 = x.parse().unwrap();
                    self.input.domain.0 = proposal.min(self.input.domain.1);
                }
                true
            }
            Msg::Right(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change right to {}", x);
                    let proposal: f64 = x.parse().unwrap();
                    self.input.domain.1 = proposal.max(self.input.domain.0);
                }
                true
            }
            Msg::ToggleFunctionFormat(index) => {
                log::trace!("Trying to change function {} format", index);
                let new_fn_input = ();
                todo!();
                true
            }
            Msg::Function(index, data) => {
                if let ChangeData::Value(f) = data {
                    log::trace!("Trying to change function {} to {}", index, f);
                    match self.input.functions[index].clone() {
                        FnInput::Analytical { .. } => {
                            self.input.functions[index] = FnInput::Analytical {
                                expression: exmex::parse(
                                    &f,
                                    exmex::make_default_operators::<f64>(),
                                )
                                .unwrap(),
                                string: f.clone(),
                            }

                            // string = f.clone();
                            // let proposal =
                            //     exmex::parse(&f, exmex::make_default_operators::<f64>()).unwrap();
                            // expression = proposal;
                        }
                        FnInput::Points(_) => todo!(),
                    }
                }
                true
            }
            Msg::AddFnInput => {
                self.input.functions.push(FnInput::default());
                true
            }

            Msg::Auxiliary(set) => match set {
                Set::TitleString(data) => {
                    if let ChangeData::Value(s) = data {
                        self.input.title_string = s;
                    }
                    true
                }
                Set::Title => {
                    self.input.title = !self.input.title;
                    true
                }
                Set::Mesh => {
                    self.input.mesh = !self.input.mesh;
                    true
                }
                Set::XAxis => {
                    self.input.x_axis = !self.input.x_axis;
                    true
                }
                Set::YAxis => {
                    self.input.y_axis = !self.input.y_axis;
                    true
                }
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // Form from yew?
                <p>{ "Main settings" }</p>

                <p>
                    { "Input the functions to plot." }
                    <button type="button" id="add_fn_input" name="add_fn_input" onclick=self.link.callback(|_| Msg::AddFnInput)>{ "Add another" }</button>
                </p>
                { for (0..self.input.functions.len()).map(|index| self.html_fn_input(index)) }


                <p>{ "Select the domain." }</p>
                <input type="number" id="left" name="left" value=self.input.domain.0.to_string() max=self.input.domain.1.to_string() step=0.1 onchange=self.link.callback(|x| Msg::Left(x))/>
                <input type="number" id="right" name="right" value=self.input.domain.1.to_string() min=self.input.domain.0.to_string() step=0.1 onchange=self.link.callback(|x| Msg::Right(x))/>

                <br/>

                <canvas ref={self.canvas_ref.clone()} />

                <p>{ "Auxiliary settings" }</p>

                <input type="checkbox" id="title" name="title" checked=self.input.title onchange=self.link.callback(|_| Msg::Auxiliary(Set::Title))/>
                { "Title" } <input type="text" id="title_string" name="title_string" value=self.input.title_string.clone() onchange=self.link.callback(|s| Msg::Auxiliary(Set::TitleString(s)))/>

                <input type="checkbox" id="mesh" name="mesh" checked=self.input.mesh onchange=self.link.callback(|_| Msg::Auxiliary(Set::Mesh))/>
                { "Mesh" }

                <input type="checkbox" id="x_axis" name="x_axis" checked=self.input.x_axis onchange=self.link.callback(|_| Msg::Auxiliary(Set::XAxis))/>
                { "X-Axis" }

                <input type="checkbox" id="y_axis" name="y_axis" checked=self.input.y_axis onchange=self.link.callback(|_| Msg::Auxiliary(Set::YAxis))/>
                { "Y-Axis" }


                <footer id="footer" name="footnote">
                <p id="authorship" name="authorship">
                    { "Author: " }<a href="https://saona-raimundo.github.io/">{ "Raimundo Saona" }</a>
                </p>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn html_fn_input(&self, index: usize) -> Html {
        let fn_string = match &self.input.functions[index] {
            FnInput::Analytical { string, .. } => string.clone(),
            FnInput::Points(_) => "points".to_string(),
        };
        let example_string = match &self.input.functions[index] {
            FnInput::Analytical { .. } => "(with curly brackets for the variable x)",
            FnInput::Points(_) => "(with format [(x1, y1), (x2, y2), ..., (xn, yn)])",
        };

        html! {
            <>
                <input type="text" id="fn_input" name="fn_input" autofocus=true value=fn_string onchange=self.link.callback(move |f| Msg::Function(index, f))/>
                <select name="cars" id="cars" onchange=self.link.callback(|_| Msg::ToggleFunctionFormat(0))>
                  <option value="functional">{ "Function" }</option>
                  <option value="points">{ "Points" }</option>
                </select>
                { example_string }
                <br/>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
