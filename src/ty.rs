use std::fmt::Display;

/// This is in a speical format
/// when apply (e.g. `F2 F1`)
/// The type will be
/// `[F1's in, F2's in] -> [F2's out, F1's out]`
///
/// The "top" of the input/output will be the first thing be used.
/// so in the vec look like this:
///
/// `Bottom -- Vec -- Top`
/// `[F2's in , F1's in ]`
/// `[F1's out, F2's out]`
#[derive(Clone, Debug, Default)]
pub struct Ty {
    pub inputs: Vec<Primitive>,
    pub outputs: Vec<Primitive>,
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inputs = self.inputs.iter().rev().fold("".to_owned(), |acc, x| {
            if acc.is_empty() {
                x.to_string()
            } else {
                format!("{acc}, {x}")
            }
        });
        let outputs = self.outputs.iter().rev().fold("".to_owned(), |acc, x| {
            if acc.is_empty() {
                x.to_string()
            } else {
                format!("{acc}, {x}")
            }
        });

        let inputs = inputs
            .is_empty()
            .then_some("ε".to_owned())
            .unwrap_or(inputs);
        write!(f, "{inputs} -> {outputs}")
    }
}

impl Ty {
    pub fn unknown_fn() -> Self {
        Self {
            inputs: vec![Primitive::Unknown],
            outputs: vec![Primitive::Unknown],
        }
    }

    pub fn unknown() -> Self {
        Self {
            outputs: vec![Primitive::Unknown],
            ..Default::default()
        }
    }

    pub fn void() -> Self {
        Default::default()
    }

    pub fn bool() -> Self {
        Self {
            outputs: vec![Primitive::Bool],
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TyId(pub usize);

#[derive(Clone, Copy, Debug, Default)]
pub enum Primitive {
    Int,
    Float,
    Void,
    Bool,

    // for type casting
    Any,
    /// Unknown never equal
    #[default]
    Unknown,
}

impl Primitive {
    pub fn is_unknown(&self) -> bool {
        if let Primitive::Unknown = *self {
            true
        } else {
            false
        }
    }

    pub fn is_any(&self) -> bool {
        if let Primitive::Any = *self {
            true
        } else {
            false
        }
    }

    pub fn is_mono(&self) -> bool {
        if self.is_any() || self.is_unknown() {
            false
        } else {
            true
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Primitive::Int => "int",
            Primitive::Float => "float",
            Primitive::Void => "ε",
            Primitive::Bool => "bool",
            Primitive::Any => "*",
            Primitive::Unknown => "?",
        };

        f.write_str(s)
    }
}

impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Primitive::Unknown) | (Primitive::Unknown, _) => false,
            (_, Primitive::Any) | (Primitive::Any, _) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Clone)]
pub enum TyRaw {
    Kind(TyKind),
    Arrow(TyKind, Box<TyRaw>),
}

impl Default for TyRaw {
    fn default() -> Self {
        TyRaw::Kind(TyKind::Primitive(Primitive::Unknown))
    }
}

#[derive(Clone)]
pub enum TyKind {
    Primitive(Primitive),
    Type(String),
}

impl Default for TyKind {
    fn default() -> Self {
        TyKind::Primitive(Default::default())
    }
}

#[derive(Clone, Copy)]
pub enum Number {
    Int(i32),
    Float(f32),
}

impl Default for Number {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl Number {
    /// will panic if wrong format
    pub fn parse_int(text: &str) -> Self {
        let int: i32 = text.parse().unwrap();
        Self::Int(int)
    }

    /// will panic if wrong format
    pub fn parse_float(text: &str) -> Self {
        let float: f32 = text.parse().unwrap();
        Self::Float(float)
    }
}
