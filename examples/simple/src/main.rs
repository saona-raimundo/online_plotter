use core::str::FromStr;
use itertools::Itertools;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

use web_sys::HtmlCanvasElement;
use yew::prelude::*;

mod input;
use input::{FnInput, FnInputKind, Input};

enum Msg {
    Function(ChangeData),
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
            input: Input::default(),
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
            Msg::Function(data) => {
                if let ChangeData::Value(mut f) = data {
                    log::trace!("Trying to change function to {}", f);
                    let kind = match FnInputKind::from_str(&f) {
                        Ok(k) => k,
                        Err(e) => {
                            log::error!("{}\nInput: {}", e, f);
                            log::warn!("Function changed to default input.");
                            let fn_input = FnInput::default();
                            f = fn_input.string.clone();
                            fn_input.kind().clone()
                        }
                    };
                    self.input.function_input.set_kind(kind).set_string(f);
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
            <>
                <p>
                    { "Input the function to plot." }
                    <div class="tooltip">{ "Available fomats?" }
                        <span class="tooltiptext">{ "analytical: sin({x})\npoints: [(0, 2), (1, 3.5)]" }</span>
                    </div>
                </p>
                { self.html_fn_input() }

                <br/>

                <canvas ref={ self.canvas_ref.clone() } />

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
    fn html_fn_input(&self) -> Html {
        let fn_input = &self.input.function_input;

        let label = "fn_input";

        let fn_string = fn_input.string.clone();

        html! {
            <>
                <input type="text" id={ label.clone() } name={ label } autofocus=true value=fn_string onchange=self.link.callback(move |f| Msg::Function(f))/>
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

        let function_input = &self.input.function_input;
        if function_input.show() {
            match function_input.kind() {
                FnInputKind::Analytical { expression, .. } => {
                    let values: Vec<f64> = grid.clone().map(|x| expression.eval(&[x])).collect();
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
