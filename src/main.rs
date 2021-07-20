use exmex::FlatEx;
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

const CANVAS_WIDTH: u32 = 720;
const CANVAS_HEIGHT: u32 = 720;
const GRID_POINTS: usize = 100;

#[derive(Debug)]
struct Input {
    domain: (f64, f64),
    math_expression: FlatEx<f64>,
    math_string: String,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            domain: (-3.14, 3.14),
            math_expression: exmex::parse("sin({x})", exmex::make_default_operators::<f64>())
                .unwrap(),
            math_string: "sin({x})".to_string(),
        }
    }
}

enum Msg {
    Left(ChangeData),
    Right(ChangeData),
    Function(ChangeData),
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
            let values: Vec<f64> = grid
                .clone()
                .map(|x| self.input.math_expression.eval(&[x]))
                .collect();
            let (min, max) = values.iter().minmax().into_option().unwrap();
            let delta = max - min;

            let mut chart = ChartBuilder::on(&root)
                // title
                .caption("Your function", ("Arial", 30))
                // enables Y axis, the size is 40 px
                .set_label_area_size(LabelAreaPosition::Left, 40)
                // enable X axis, the size is 40 px
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                // build cartesion chart
                .build_cartesian_2d(
                    self.input.domain.0..self.input.domain.1,
                    (min - delta / 100.)..(max + delta / 100.),
                )
                .unwrap();

            // Mesh
            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(grid.zip(values), &BLACK))
                .unwrap();
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
            Msg::Function(data) => {
                if let ChangeData::Value(f) = data {
                    log::trace!("Trying to change function to {}", f);
                    self.input.math_string = f.clone();
                    let proposal =
                        exmex::parse(&f, exmex::make_default_operators::<f64>()).unwrap();
                    self.input.math_expression = proposal; //f; //mexprp::Expression::parse(f).unwrap();
                }
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                // Form from yew?
                <p>{ "Input the function (with curly brackets for the variable x)." }</p>
                <input type="text" id="function" name="function" autofocus=true value=self.input.math_string.clone() onchange=self.link.callback(|f| Msg::Function(f))/>
                <p>{ "Select the domain." }</p>
                <input type="number" id="left" name="left" value=self.input.domain.0.to_string() max=self.input.domain.1.to_string() step=0.1 onchange=self.link.callback(|x| Msg::Left(x))/>
                <input type="number" id="right" name="right" value=self.input.domain.1.to_string() min=self.input.domain.0.to_string() step=0.1 onchange=self.link.callback(|x| Msg::Right(x))/>
                <p></p>
                <canvas ref={self.canvas_ref.clone()} />
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
