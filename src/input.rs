use exmex::FlatEx;
use splines::Spline;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct FnInput {
    pub show: bool,
    pub string: String,
    pub kind: FnInputKind,
}

#[derive(Debug, Clone)]
pub enum FnInputKind {
    Analytical { expression: FlatEx<f64> },
    Points { spline: Spline<f64, f64> }, //values:  Vec<(f64, f64)> },
}
impl Default for FnInputKind {
    fn default() -> Self {
        let string = "sin({x})".to_string();
        FnInputKind::Analytical {
            expression: exmex::parse::<f64>(&string, exmex::make_default_operators::<f64>())
                .unwrap(),
        }
    }
}

impl Default for FnInput {
    fn default() -> Self {
        let string = "sin({x})".to_string();
        FnInput {
            string: string,
            show: true,
            kind: FnInputKind::default(),
        }
    }
}

impl FnInput {
    pub fn show(&self) -> bool {
        self.show
    }
    pub fn toggle(&mut self) -> &mut Self {
        log::trace!("Toggling a fn_input");
        self.show = !self.show;
        log::trace!("Now show is {}", self.show());
        self
    }
    pub fn kind(&self) -> &FnInputKind {
        &self.kind
    }
    // pub fn kind_mut(&mut self) -> &mut FnInputKind {
    //     &mut self.kind
    // }
    pub fn set_kind(&mut self, kind: FnInputKind) -> &mut Self {
        self.kind = kind;
        self
    }
    pub fn set_string(&mut self, s: String) -> &mut Self {
        self.string = s;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle() {
        let mut fn_input = FnInput::default();
        assert_eq!(fn_input.show(), !fn_input.toggle().show());
    }
}

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
