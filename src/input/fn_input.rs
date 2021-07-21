use core::fmt::Display;
use core::str::FromStr;
use exmex::FlatEx;
use splines::{interpolation::Interpolation, key::Key, spline::Spline};
use thiserror::Error;

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
#[derive(Error, Debug)]
pub struct FormatError(String);
impl Display for FormatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "The input was not an anlytical function nor a collection of points."
        )
    }
}

impl FromStr for FnInputKind {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, FormatError> {
        if let Ok(expression) = exmex::parse(s, exmex::make_default_operators::<f64>()) {
            log::debug!("We noticed an analyical function :)");
            Ok(FnInputKind::Analytical { expression })
        } else {
            if let Ok(values) = ron::de::from_str::<Vec<(f64, f64)>>(s) {
                let spline = Spline::from_iter(
                    values
                        .iter()
                        .map(|(x, y)| Key::new(*x, *y, Interpolation::Cosine)),
                );
                log::debug!("We noticed a collection of points");
                Ok(FnInputKind::Points { spline })
            } else {
                Err(FormatError(s.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnInput {
    pub show: bool,
    pub string: String,
    pub kind: FnInputKind,
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
