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
    pub function_input: FnInput,
}

impl Default for Input {
    fn default() -> Self {
        let function_input = FnInput::default();
        Self {
            canvas_size: (360, 360),
            domain: (-3.14, 3.14),
            mesh: true,
            x_axis: true,
            y_axis: true,
            title: true,
            title_string: "Your function".to_string(),
            quality: 100,
            function_input,
        }
    }
}
