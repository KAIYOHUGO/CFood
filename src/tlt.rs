use std::{collections::BTreeMap, fmt::Display};

use crate::ty::*;

#[derive(Clone, Debug)]
pub struct TLT {
    id: usize,
    ty_map: BTreeMap<TyId, Ty>,
    alias: BTreeMap<String, TyId>,
    block: Vec<BTreeMap<String, TyId>>,
    error: bool,
}

impl TLT {
    pub fn new_ty(&mut self, ty: Ty) -> TyId {
        let id = self.new_id();
        self.ty_map.insert(id, ty);
        return id;
    }

    pub fn ty(&self, id: TyId) -> Ty {
        self.ty_map.get(&id).unwrap().clone()
    }

    pub fn ty_mut(&mut self, id: TyId) -> &mut Ty {
        self.ty_map.get_mut(&id).unwrap()
    }

    fn ty_raw_to_ty(&mut self, ty_raw: &TyRaw, line: u32) -> Ty {
        let (inputs, outputs) = match ty_raw {
            TyRaw::Kind(ty_kind) => match ty_kind {
                TyKind::Primitive(primitive) => (vec![], vec![*primitive]),
                TyKind::Type(name) => {
                    // alias is a output only type
                    let outputs = self.alias(name, line).outputs;
                    (vec![], outputs)
                }
            },
            TyRaw::Arrow(ty_kind, ty_raw) => {
                let inputs = match ty_kind {
                    TyKind::Primitive(primitive) => vec![*primitive],
                    TyKind::Type(name) => {
                        // alias is a output only type
                        let outputs = self.alias(name, line).outputs;
                        outputs
                    }
                };
                let ty = self.ty_raw_to_ty(ty_raw, line);
                let inputs = [ty.inputs, inputs].concat();
                (inputs, ty.outputs)
            }
        };
        Ty { inputs, outputs }
    }

    // return type id
    pub fn new_fn(&mut self, name: &str, ret: &TyRaw, params: &Vec<TyId>, line: u32) -> TyId {
        let ty = self.ty_raw_to_ty(ret, line);

        // Magic name "return"
        let ret_id = self.new_ty(ty.clone());
        self.block
            .last_mut()
            .unwrap()
            .insert("return".to_owned(), ret_id);

        let mut inputs = ty.inputs;
        for param in params.into_iter().rev() {
            let param = self.ty(*param);
            inputs.extend(param.outputs);

            self.assert(
                param.inputs.is_empty(),
                line,
                "Input type should be order 0",
            );
        }

        let id = self.new_ty(Ty {
            inputs,
            outputs: ty.outputs,
        });
        self.block.first_mut().unwrap().insert(name.to_owned(), id);

        id
    }

    pub fn new_var_raw(&mut self, name: &str, ty_raw: &TyRaw, line: u32) -> TyId {
        let ty = self.ty_raw_to_ty(ty_raw, line);
        let id = self.new_ty(ty);
        let out = self.block.last_mut().unwrap().insert(name.to_owned(), id);
        if out.is_some() {
            self.error(line, format!("Duplicate declare: {}", name));
        }

        id
    }

    pub fn new_var_arr(&mut self, name: &str, ty_kind: &TyKind, num: &Number, line: u32) -> TyId {
        let outputs = match ty_kind {
            TyKind::Primitive(primitive) => vec![*primitive],
            TyKind::Type(name) => self.alias(name, line).outputs,
        };
        let outputs = match num {
            Number::Int(size) if *size >= 0 => outputs.repeat(*size as usize),
            Number::Int(_) => {
                self.error(line, "Negative int is not a vaild size");
                vec![Primitive::Unkonwn]
            }
            Number::Float(_) => {
                self.error(line, "Float is not a vaild size");
                vec![Primitive::Unkonwn]
            }
        };
        let id = self.new_ty(Ty {
            outputs,
            ..Default::default()
        });
        let out = self.block.last_mut().unwrap().insert(name.to_owned(), id);
        if out.is_some() {
            self.error(line, format!("Duplicate declare: {}", name));
        }

        id
    }

    pub fn new_let(&mut self, name: &str, ty_id: TyId, line: u32) -> TyId {
        let out = self
            .block
            .last_mut()
            .unwrap()
            .insert(name.to_owned(), ty_id);
        if out.is_some() {
            self.error(line, format!("Duplicate declare: {}", name));
        }

        ty_id
    }
    pub fn var(&mut self, name: &str, line: u32) -> TyId {
        for scope in self.block.iter().rev() {
            if let Some(id) = scope.get(name) {
                return *id;
            }
        }
        self.error(line, format!("Cannot find `{name}` in scope"));
        self.new_ty(Ty::unknown())
    }

    pub fn var_arr(&mut self, name: &str, line: u32) -> TyId {
        let res = self
            .block
            .iter()
            .rev()
            .filter_map(|scope| scope.get(name))
            .next();
        let Some(id) = res else {
            self.error(line, format!("Cannot find `{name}` in scope"));
            return self.new_ty(Ty::unknown());
        };

        let ty = self.ty(*id);
        let index_ty = ty
            .outputs
            .iter()
            .map(|x| *x)
            .reduce(|acc, e| if acc != e { Primitive::Unkonwn } else { acc });
        let is_arr = ty.inputs.is_empty()
            && !ty.outputs.is_empty()
            && index_ty.map(|x| !x.is_unknown()).unwrap_or_default();

        if !is_arr {
            self.error(line, format!("Varible `{name}` is not indexable"));
            return self.new_ty(Ty::unknown());
        }

        return self.new_ty(Ty {
            outputs: vec![index_ty.unwrap()],
            ..Default::default()
        });
    }
    pub fn new_alias(&mut self, name: String, kinds: &Vec<TyKind>, line: u32) {
        let mut outputs = vec![];
        for kind in kinds.into_iter().rev() {
            match kind {
                TyKind::Primitive(primitive) => match primitive {
                    Primitive::Void => {}
                    _ => outputs.push(*primitive),
                },
                TyKind::Type(name) => {
                    let ty = self.alias(&name, line);

                    // alias is a output only type
                    for t in ty.outputs {
                        outputs.push(t);
                    }
                }
            }
        }

        let id = self.new_ty(Ty {
            outputs,
            ..Default::default()
        });
        let out = self.alias.insert(name.clone(), id);
        if out.is_some() {
            self.error(line, format!("Duplicate declare: {}", name));
        }
    }

    pub fn alias(&mut self, name: &String, line: u32) -> Ty {
        let Some(id) = self.alias.get(name) else {
            self.error(line, format!("Cannot find type alias '{name}'"));
            return Ty::unknown_fn();
        };

        self.ty(*id)
    }

    pub fn new_ty_from_number(&mut self, num: Number) -> TyId {
        match num {
            Number::Int(_) => self.new_ty(Ty {
                outputs: vec![Primitive::Int],
                ..Default::default()
            }),
            Number::Float(_) => self.new_ty(Ty {
                outputs: vec![Primitive::Float],
                ..Default::default()
            }),
        }
    }

    pub fn binary_op(&mut self, lhs: TyId, rhs: TyId, line: u32) -> TyId {
        let lhs = self.ty(lhs);
        let rhs = self.ty(rhs);
        if !lhs.inputs.is_empty()
            || lhs.outputs.len() != 1
            || !rhs.inputs.is_empty()
            || rhs.outputs.len() != 1
        {
            self.error(
                line,
                format!("The type {lhs} and {rhs} is not allow in binary op"),
            );
            return self.new_ty(Ty::unknown());
        }

        let out = self.cast_type(lhs.outputs[0], rhs.outputs[0], line);
        self.new_ty(Ty {
            outputs: vec![out],
            ..Default::default()
        })
    }

    pub fn cmp_op(&mut self, lhs: TyId, rhs: TyId, line: u32) -> TyId {
        let lhs = self.ty(lhs);
        let rhs = self.ty(rhs);
        if !lhs.inputs.is_empty()
            || lhs.outputs.len() != 1
            || !rhs.inputs.is_empty()
            || rhs.outputs.len() != 1
        {
            self.error(
                line,
                format!("The type {lhs} and {rhs} is not allow in binary op"),
            );
            return self.new_ty(Ty::unknown());
        }

        let _ = self.cast_type(lhs.outputs[0], rhs.outputs[0], line);
        self.new_ty(Ty {
            outputs: vec![Primitive::Bool],
            ..Default::default()
        })
    }
    pub fn apply(&mut self, lhs: TyId, rhs: TyId, line: u32) -> TyId {
        let mut lhs = self.ty(lhs);
        let mut rhs = self.ty(rhs);

        let mut error = false;
        while !lhs.inputs.is_empty() && !rhs.outputs.is_empty() {
            let li = lhs.inputs.pop().unwrap();
            let ro = rhs.outputs.pop().unwrap();

            let out = self.cast_type(li, ro, line);
            if out.is_unknown() {
                self.error(line, format!("Type mismatch: {} != {}", li, ro));
                error = true;
            }

            // TODO:cascade the tyoe change
        }

        let ty = if error {
            Ty {
                inputs: vec![Primitive::Unkonwn],
                outputs: [rhs.outputs, lhs.outputs].concat(),
            }
        } else {
            Ty {
                inputs: [lhs.inputs, rhs.inputs].concat(),
                outputs: [rhs.outputs, lhs.outputs].concat(),
            }
        };
        self.new_ty(ty)
    }

    pub fn enter_block(&mut self) {
        self.block.push(Default::default());
    }

    pub fn exit_block(&mut self) {
        self.block.pop();
    }

    pub fn cast_type(&mut self, lhs: Primitive, rhs: Primitive, line: u32) -> Primitive {
        let lhs_is_mono = lhs.is_mono();
        let rhs_is_mono = rhs.is_mono();

        match (lhs_is_mono, rhs_is_mono) {
            (true, false) => lhs,
            (false, true) => rhs,
            _ if lhs == rhs => lhs,
            _ => {
                self.error(line, format!("Type cast fail, {lhs} cannot cast to {rhs}"));
                Primitive::Unkonwn
            }
        }
    }

    pub fn assert_ty_id(&mut self, lhs: TyId, rhs: TyId, line: u32) {
        let lhs = self.ty(lhs);
        let rhs = self.ty(rhs);

        let mut error = false;
        for (lhs, rhs) in lhs.inputs.iter().zip(rhs.inputs.iter()) {
            error = error || self.cast_type(*lhs, *rhs, line).is_unknown();
        }
        for (lhs, rhs) in lhs.outputs.iter().zip(rhs.outputs.iter()) {
            error = error || self.cast_type(*lhs, *rhs, line).is_unknown();
        }

        if error {
            self.error(line, format!("Type bound fail, {lhs} cannot cast to {rhs}"));
        }
    }

    fn assert(&mut self, cond: bool, line: u32, msg: impl Display) -> bool {
        if !cond {
            self.error(line, msg);
        }

        cond
    }

    fn error(&mut self, line: u32, msg: impl Display) {
        self.error = true;
        println!("==Error== {line}: {msg}")
    }

    fn new_id(&mut self) -> TyId {
        let ret = TyId(self.id);
        self.id += 1;
        return ret;
    }
}

impl Default for TLT {
    fn default() -> Self {
        Self {
            id: 0,
            ty_map: Default::default(),
            alias: Default::default(),
            block: vec![Default::default()],
            error: false,
        }
    }
}
