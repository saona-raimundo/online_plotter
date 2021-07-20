use exmex::FlatEx;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub enum FnInput {
    Analytical {
        expression: FlatEx<f64>,
        string: String,
    },
    Points(Vec<(f64, f64)>),
}
impl Default for FnInput {
    fn default() -> Self {
        FnInput::Analytical {
            expression: exmex::parse("sin({x})", exmex::make_default_operators::<f64>()).unwrap(),
            string: "sin({x})".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub domain: (f64, f64),
    pub mesh: bool,
    pub x_axis: bool,
    pub y_axis: bool,
    pub title: bool,
    pub title_string: String,
    pub functions: Vec<FnInput>,
}

impl Default for Input {
    fn default() -> Self {
        let functions = vec![FnInput::default()];
        Self {
            domain: (-3.14, 3.14),
            mesh: true,
            x_axis: true,
            y_axis: true,
            title: true,
            title_string: "Your function".to_string(),
            functions,
        }
    }
}

#[derive(Debug)]
pub enum Set {
    TitleString(ChangeData),
    Mesh,
    XAxis,
    YAxis,
    Title,
}
