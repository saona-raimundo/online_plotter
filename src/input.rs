use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

mod fn_input;
pub use fn_input::{FnInput, FnInputKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

const KEY: &'static str = "online_plotter.v.0.1.1.input";
impl Input {
    pub fn restore_or_default() -> Self {
        log::trace!("Restoring values from storage");
        if let Ok(storage) = StorageService::new(Area::Local) {
            storage.restore(KEY)
        } else {
            Self::default()
        }
    }
    pub fn restore() -> Self {
        log::trace!("Restoring values from storage");
        let storage = StorageService::new(Area::Local).unwrap();
        storage.restore(KEY)
    }

    pub fn store(&self) {
        log::trace!("Storing values into storage");
        let mut storage = StorageService::new(Area::Local).unwrap();
        storage.store(KEY, self);
    }
}

impl<'a> Default for Input {
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

impl From<yew::format::Text> for Input {
    fn from(text: yew::format::Text) -> Self {
        log::trace!("Transforming text into Input, source {:?}", text);
        match text {
            Ok(string) => {
                log::trace!("Found previous record! {}", string);
                ron::de::from_str(&string).unwrap()
            }
            Err(e) => {
                log::error!("Failed to find previous record! {:?}", e);
                log::warn!("Input changed to default.");
                Self::default()
            }
        }
    }
}

impl Into<yew::format::Text> for &Input {
    fn into(self) -> yew::format::Text {
        log::trace!("Transforming Input into Text");
        Ok(ron::ser::to_string(self)?)
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
    pub fn update_and_store(&mut self, set: Set) -> ShouldRender {
        let should_render = self.update(set);
        self.store();
        should_render
    }
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
