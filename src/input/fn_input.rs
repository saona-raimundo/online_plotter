use core::fmt::Display;
use core::str::FromStr;
use exmex::OwnedFlatEx;
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use splines::{interpolation::Interpolation, key::Key, spline::Spline};
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum FnInputKind {
    Analytical { expression: OwnedFlatEx<f64> },
    Points { spline: Spline<f64, f64> },
}
impl Default for FnInputKind {
    fn default() -> Self {
        let string = "sin({x})".to_string();
        FnInputKind::Analytical {
            expression: OwnedFlatEx::from_flatex(
                exmex::parse::<f64>(&string, &exmex::make_default_operators::<f64>()).unwrap(),
            ),
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
        let string = s.to_string();
        if let Ok(expr) = exmex::parse_with_default_ops(&string) {
            log::debug!("We noticed an analyical function :)");
            let expression = OwnedFlatEx::from_flatex(expr);
            Ok(FnInputKind::Analytical { expression })
        } else {
            log::debug!("We noticed it was not an analyical function.");
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// impl Serialize for FnInput {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("FnInput", 2)?;
//         s.serialize_field("show", &self.show)?;
//         s.serialize_field("string", &self.string)?;
//         s.end()
//     }
// }

// impl<'de> Deserialize<'de> for FnInput {
//     fn deserialize<D>(deserializer: D) -> Result<FnInput, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Debug, Clone, Deserialize)]
//         pub struct __FnInput {
//             pub show: bool,
//             pub string: String,
//         }
//         let __fn_input = __FnInput::deserialize(deserializer)?;
//         let kind = FnInputKind::from_str(&__fn_input.string).unwrap();
//         Ok(FnInput {
//             show: __fn_input.show,
//             string: __fn_input.string,
//             kind,
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle() {
        let mut fn_input = FnInput::default();
        assert_eq!(fn_input.show(), !fn_input.toggle().show());
    }
}
