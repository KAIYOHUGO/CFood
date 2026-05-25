use std::{collections::BTreeMap, fmt::Display};

use is_macro::Is;

use crate::error::*;

#[derive(Clone, Debug, Default)]
pub struct TypeStore {
    map: BTreeMap<TypeId, AType>,
    cst_map: BTreeMap<usize, TypeId>,
    count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId(usize);

impl TypeStore {
    fn get_id(&mut self) -> TypeId {
        let id = self.count;
        self.count += 1;
        TypeId(id)
    }

    pub fn get(&self, id: TypeId) -> &AType {
        self.map.get(&id).unwrap()
    }

    pub fn get_type_id(&self, cst_id: usize) -> Option<TypeId> {
        self.cst_map.get(&cst_id).cloned()
    }

    pub fn unknown(&mut self, cst_id: usize) -> TypeId {
        let id = self.get_id();
        self.map.insert(id, AType::Unknown(cst_id));
        self.cst_map.insert(cst_id, id);

        id
    }

    pub fn void(&mut self, cst_id: usize) -> TypeId {
        self.c_type(CType {
            cst_id,
            ..Default::default()
        })
    }

    pub fn prim(&mut self, kind: PrimKind, cst_id: usize) -> TypeId {
        let id = self.get_id();
        let prim = Prim { cst_id, kind };
        self.map.insert(
            id,
            AType::CType(CType {
                cst_id,
                outputs: vec![prim],
                ..Default::default()
            }),
        );
        self.cst_map.insert(cst_id, id);
        id
    }

    pub fn c_type(&mut self, c_type: CType) -> TypeId {
        let id = self.get_id();
        self.cst_map.insert(c_type.cst_id, id);
        self.map.insert(id, AType::CType(c_type));
        id
    }

    pub fn a_type(&mut self, a_type: AType) -> TypeId {
        let id = self.get_id();
        let cst_id = match &a_type {
            AType::CType(ctype) => ctype.cst_id,
            AType::Unknown(cst_id) => *cst_id,
        };
        self.cst_map.insert(cst_id, id);
        self.map.insert(id, a_type);
        id
    }

    pub fn apply(&mut self, lhs: TypeId, rhs: TypeId, cst_id: usize) -> CFoodResult<TypeId> {
        let lhs = self.get(lhs);
        let rhs = self.get(rhs);

        let (AType::CType(mut lhs), AType::CType(mut rhs)) = (lhs.clone(), rhs.clone()) else {
            return Ok(self.unknown(cst_id));
        };

        let mut labels = vec![];
        while !lhs.inputs.is_empty() && !rhs.outputs.is_empty() {
            let li = lhs.inputs.pop().unwrap();
            let ro = rhs.outputs.pop().unwrap();

            if li != ro {
                labels.push(CFoodErrorLabel {
                    cst_id: li.cst_id,
                    label: Some(format!("Type: {li}")),
                });
                labels.push(CFoodErrorLabel {
                    cst_id: ro.cst_id,
                    label: Some(format!("Type: {ro}")),
                });
            }
        }

        if !labels.is_empty() {
            return Err(CFoodError {
                message: "Application Type Error".to_owned(),
                labels,
                ..Default::default()
            });
        }

        let id = self.get_id();

        let res = CType {
            inputs: [lhs.inputs, rhs.inputs].concat(),
            outputs: [rhs.outputs, lhs.outputs].concat(),
            cst_id,
        };

        self.c_type(res);

        Ok(id)
    }

    pub fn is_eq(&self, a: TypeId, b: TypeId) -> bool {
        let a = self.get(a);
        let b = self.get(b);
        a == b
    }
}

#[derive(Clone, Debug, Is, PartialEq, Eq)]
pub enum AType {
    CType(CType),
    Unknown(usize),
}

impl Display for AType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AType::CType(ctype) => ctype.fmt(f),
            AType::Unknown(_) => f.write_str("?"),
        }
    }
}

/// A **C**oncrete type
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CType {
    pub cst_id: usize,
    pub inputs: Vec<Prim>,
    pub outputs: Vec<Prim>,
}

impl Display for CType {
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
            .then_some(".".to_owned())
            .unwrap_or(inputs);
        let outputs = outputs
            .is_empty()
            .then_some(".".to_owned())
            .unwrap_or(outputs);

        write!(f, "{inputs} -> {outputs}")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq)]
pub struct Prim {
    pub cst_id: usize,
    pub kind: PrimKind,
}

impl PartialEq for Prim {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Is)]
pub enum PrimKind {
    Int,
    Float,
    Bool,
    ConStr,
}

impl Display for Prim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.kind {
            PrimKind::Int => "int",
            PrimKind::Float => "float",
            PrimKind::Bool => "bool",
            PrimKind::ConStr => "str",
        };

        f.write_str(s)
    }
}
