use core::str::FromStr;
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

mod input;
use input::{FnInput, FnInputKind, Input, Set};

enum Msg {
    Left(ChangeData),
    Right(ChangeData),
    AddFnInput,
    Function(usize, ChangeData),
    ToggleFunction(usize),
    Auxiliary(Set),
}
struct Model {
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
            input: Input::restore_or_default(),
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        let canvas: HtmlCanvasElement = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(self.input.canvas_size.0);
        canvas.set_height(self.input.canvas_size.1);
        let backend: CanvasBackend = CanvasBackend::with_canvas_object(canvas).unwrap();
        self.plot(backend);
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Left(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change left to {}", x);
                    let proposal: f64 = x.parse().unwrap();
                    self.input.domain.0 = proposal.min(self.input.domain.1);
                    self.input.store();
                }
                true
            }
            Msg::Right(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change right to {}", x);
                    let proposal: f64 = x.parse().unwrap();
                    self.input.domain.1 = proposal.max(self.input.domain.0);
                    self.input.store();
                }
                true
            }
            Msg::ToggleFunction(index) => {
                log::trace!("Trying to toggle function {}", index);
                self.input.functions[index].toggle();
                log::trace!("Function {} toggled", index);
                self.input.store();
                true
            }
            Msg::Function(index, data) => {
                if let ChangeData::Value(mut f) = data {
                    log::trace!("Trying to change function index {} to {}", index, f);
                    let kind = match FnInputKind::from_str(&f) {
                        Ok(k) => {
                            log::trace!("Identified function input of kind {:?}", k);
                            k
                        }
                        Err(e) => {
                            log::error!("{}\nInput: {}", e, f);
                            log::warn!("Function changed to default input.");
                            let fn_input = FnInput::default();
                            f = fn_input.string.clone();
                            fn_input.kind().clone()
                        }
                    };
                    self.input.functions[index].set_kind(kind).set_string(f);
                    self.input.store();
                }
                true
            }
            Msg::AddFnInput => {
                self.input.functions.push(FnInput::default());
                self.input.store();
                true
            }

            Msg::Auxiliary(set) => self.input.update_and_store(set),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <canvas ref={ self.canvas_ref.clone() } />

                <div class="MainPanel">
                    <p>{ "Main settings" }</p>
                    <p>
                        { "Input the functions to plot." }
                        <div class="tooltip">{ "Available fomats?" }
                            <span class="tooltiptext">{ "analytical: sin(x)\npoints: [(0, 2), (1, 3.5)]" }</span>
                        </div>
                    </p>
                    { for (0..self.input.functions.len()).map(|index| self.html_fn_input(index)) }
                    <button type="button" id="add_fn_input" name="add_fn_input" onclick=self.link.callback(|_| Msg::AddFnInput)>{ "Add another" }</button>
                    <p>{ "Select the domain." }</p>
                    <input type="number" id="left" name="left" value=self.input.domain.0.to_string() max=self.input.domain.1.to_string() step=0.1 onchange=self.link.callback(|x| Msg::Left(x))/>
                    <input type="number" id="right" name="right" value=self.input.domain.1.to_string() min=self.input.domain.0.to_string() step=0.1 onchange=self.link.callback(|x| Msg::Right(x))/>
                </div>
                <div class="AuxiliaryPanel">
                    <p>{ "Auxiliary settings" }</p>
                    { self.html_auxiliary_settings() }
                </div>


                <footer id="footer" name="footnote">
                <p id="authorship" name="authorship">
                    { "Author: " }<a href="https://saona-raimundo.github.io/">{ "Raimundo Saona" }</a>
                </p>
                </footer>
            </>
        }
    }
}

impl Model {
    fn html_auxiliary_settings(&self) -> Html {
        html! {
            <div class="auxiliary_settings">
                <div>
                    <p>{ "Plot" }</p>
                    <input type="checkbox" id="title" name="title" checked=self.input.title onchange=self.link.callback(|_| Msg::Auxiliary(Set::Title))/>
                    { "Title" }
                    <input type="text" id="title_string" name="title_string" value=self.input.title_string.clone() onchange=self.link.callback(|s| Msg::Auxiliary(Set::TitleString(s)))/>

                    <input type="checkbox" id="mesh" name="mesh" checked=self.input.mesh onchange=self.link.callback(|_| Msg::Auxiliary(Set::Mesh))/>
                    { "Mesh" }

                    <input type="checkbox" id="x_axis" name="x_axis" checked=self.input.x_axis onchange=self.link.callback(|_| Msg::Auxiliary(Set::XAxis))/>
                    { "X-Axis" }

                    <input type="checkbox" id="y_axis" name="y_axis" checked=self.input.y_axis onchange=self.link.callback(|_| Msg::Auxiliary(Set::YAxis))/>
                    { "Y-Axis" }

                    { "Quality" }
                    <input type="range" id="quality" name="quality" min="2" max="1000" value=self.input.quality.to_string() class="slider" onchange=self.link.callback(|x| Msg::Auxiliary(Set::Quality(x)))/>
                </div>
                <div>
                    <p>{ "Canvas" }</p>
                    { "width" }
                    <input type="range" id="canvas_width" name="canvas_width" min="5" max="1600" value=self.input.canvas_size.0.to_string() class="slider" onchange=self.link.callback(|x| Msg::Auxiliary(Set::CanvasWidth(x)))/>

                    { "height" }
                    <input type="range" id="canvas_height" name="canvas_height" min="5" max="1600" value=self.input.canvas_size.1.to_string() class="slider" onchange=self.link.callback(|x| Msg::Auxiliary(Set::CanvasHeight(x)))/>

                </div>
            </div>
        }
    }

    fn html_fn_input(&self, index: usize) -> Html {
        let fn_input = &self.input.functions[index];

        let label = format!("fn_input_{}", index);

        let fn_string = fn_input.string.clone();

        html! {
            <>
                <input type="checkbox" id="y_axis" name="y_axis" checked=fn_input.show() onchange=self.link.callback(move |_| Msg::ToggleFunction(index))/>
                <input type="text" id={ label.clone() } name={ label } autofocus=true value=fn_string onchange=self.link.callback(move |f| Msg::Function(index, f))/>
                <br/>
            </>
        }
    }

    fn plot(&self, backend: CanvasBackend) {
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let grid =
            itertools_num::linspace(self.input.domain.0, self.input.domain.1, self.input.quality);

        let mut values_collection = vec![];
        let mut overall_min = f64::INFINITY;
        let mut overall_max = f64::NEG_INFINITY;

        if self
            .input
            .functions
            .iter()
            .all(|function_input| !function_input.show())
        {
            log::trace!("There is no function to plot.");
            overall_min = -1.;
            log::warn!("min value changed to {}", overall_min);
            overall_max = 1.;
            log::warn!("max value changed to {}", overall_max);
        } else {
            for function_input in &self.input.functions {
                if function_input.show() {
                    log::trace!("Computing values for function {:?}", function_input);
                    match function_input.kind() {
                        FnInputKind::Analytical { expression, .. } => {
                            let values: Vec<f64> = grid
                                .clone()
                                .map(|x| expression.eval(&[x]).unwrap())
                                .collect();
                            let (min, max) = values.iter().minmax().into_option().unwrap();
                            overall_min = min.min(overall_min);
                            overall_max = max.max(overall_max);
                            values_collection.push(values);
                        }
                        FnInputKind::Points { spline } => {
                            let values: Vec<f64> = grid
                                .clone()
                                .map(|x| spline.clamped_sample(x).unwrap())
                                .collect();
                            let (min, max) = values.iter().minmax().into_option().unwrap();
                            overall_min = min.min(overall_min);
                            overall_max = max.max(overall_max);
                            values_collection.push(values);
                        }
                    }
                }
            }

            log::trace!(
                "Num max/min values of the plot: ({}, {})",
                overall_max,
                overall_min
            );

            if !overall_min.is_finite() {
                log::error!("min value is not real!");
                overall_min = -1.;
                log::warn!("min value changed to {}", overall_min);
            }
            if !overall_max.is_finite() {
                log::error!("max value is not real!");
                overall_max = 1.;
                log::warn!("max value changed to {}", overall_max);
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

        if !values_collection.is_empty() {
            for values in values_collection {
                chart
                    .draw_series(LineSeries::new(grid.clone().zip(values), &BLACK))
                    .unwrap();
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
