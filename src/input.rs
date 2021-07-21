use yew::prelude::*;

mod fn_input;
pub use fn_input::{FnInput, FnInputKind};

#[derive(Debug, Clone)]
pub struct Input {
    pub canvas_size: (u32, u32),
    pub domain: (f64, f64),
    pub mesh: bool,
    pub x_axis: bool,
    pub y_axis: bool,
    pub title: bool,
    pub title_string: String,
    pub quality: usize,
    pub functions: Vec<FnInput>,
}

impl Default for Input {
    fn default() -> Self {
        let functions = vec![FnInput::default()];
        Self {
            canvas_size: (360, 360),
            domain: (-3.14, 3.14),
            mesh: true,
            x_axis: true,
            y_axis: true,
            title: true,
            title_string: "Your function".to_string(),
            quality: 100,
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
    Quality(ChangeData),
    CanvasWidth(ChangeData),
    CanvasHeight(ChangeData),
}

impl Input {
    pub fn update(&mut self, set: Set) -> ShouldRender {
        match set {
            Set::TitleString(data) => {
                if let ChangeData::Value(s) = data {
                    self.title_string = s;
                }
                true
            }
            Set::Title => {
                self.title = !self.title;
                true
            }
            Set::Mesh => {
                self.mesh = !self.mesh;
                true
            }
            Set::XAxis => {
                self.x_axis = !self.x_axis;
                true
            }
            Set::YAxis => {
                self.y_axis = !self.y_axis;
                true
            }
            Set::CanvasWidth(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change canvas width to {}", x);
                    let proposal: u32 = x.parse().unwrap();
                    self.canvas_size.0 = proposal;
                }
                true
            }
            Set::CanvasHeight(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change canvas width to {}", x);
                    let proposal: u32 = x.parse().unwrap();
                    self.canvas_size.1 = proposal;
                }
                true
            }
            Set::Quality(data) => {
                if let ChangeData::Value(x) = data {
                    log::trace!("Trying to change quality to {}", x);
                    let proposal: usize = x.parse().unwrap();
                    self.quality = proposal;
                }
                true
            }
        }
    }
}
