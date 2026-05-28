use std::collections::BTreeMap;

use from_variant::FromVariant;
use inkwell::{
    types::{FunctionType, StructType},
    values::{FunctionValue, PointerValue},
};
use is_macro::Is;

use crate::checker::TypeId;

pub struct LLVMVarStore<'ctx> {
    map: BTreeMap<usize, LLVMVar<'ctx>>,
    refer_map: BTreeMap<usize, usize>,
}

impl<'ctx> LLVMVarStore<'ctx> {
    pub fn new(refer_map: BTreeMap<usize, usize>) -> Self {
        Self {
            map: Default::default(),
            refer_map,
        }
    }
    pub fn new_value(&mut self, cst_id: usize, value: LLVMValue<'ctx>) {
        self.map.insert(cst_id, value.into());
    }

    pub fn new_func(&mut self, cst_id: usize, func: LLVMFunc<'ctx>) {
        self.map.insert(cst_id, func.into());
    }

    pub fn get(&self, cst_id: usize) -> &LLVMVar<'ctx> {
        match self.map.get(&cst_id) {
            Some(var) => var,
            None => self.map.get(&self.refer_map[&cst_id]).unwrap(),
        }
    }
    pub fn get_mut(&mut self, cst_id: usize) -> &mut LLVMVar<'ctx> {
        let key = self.refer_map.get(&cst_id).copied().unwrap_or(cst_id);
        self.map.get_mut(&key).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Is, FromVariant)]
pub enum LLVMVar<'ctx> {
    Value(LLVMValue<'ctx>),
    Func(LLVMFunc<'ctx>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LLVMValue<'ctx> {
    pub id: TypeId,
    pub ty: StructType<'ctx>,
    pub value: PointerValue<'ctx>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LLVMFunc<'ctx> {
    pub id: TypeId,
    pub ty: FunctionType<'ctx>,
    pub func: FunctionValue<'ctx>,
}
