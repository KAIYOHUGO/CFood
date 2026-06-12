use std::collections::BTreeMap;

use crate::{
    checker::{CType, Prim, PrimKind, TLT, TLTAlias},
    cst::{
        Marked,
        tys::{Decl, DeclAlias, DeclFunc, DeclVar, Kind},
    },
    error::*,
};

use anyhow::Result;

pub fn hoist_alias<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b Vec<Decl>) {
    // Hoisting
    let max_path = n.len();
    let mut lookup_alias = BTreeMap::new();
    for decl in n {
        match decl {
            Decl::Alias(decl_alias) => {
                lookup_alias.insert(decl_alias.name.inner.as_str(), decl_alias);
            }
            _ => {}
        }
    }
    fn helper(
        alias: &DeclAlias,
        lookup_alias: &BTreeMap<&str, &DeclAlias>,
        remain: usize,
    ) -> Result<Vec<Prim>, ()> {
        if remain == 0 {
            return Err(());
        }
        let mut outputs = vec![];
        for kind in &alias.kinds {
            match kind {
                Kind::Int(id) => {
                    outputs.push(Prim {
                        cst_id: id.0,
                        kind: PrimKind::Int,
                    });
                }
                Kind::Float(id) => {
                    outputs.push(Prim {
                        cst_id: id.0,
                        kind: PrimKind::Float,
                    });
                }
                Kind::Bool(id) => {
                    outputs.push(Prim {
                        cst_id: id.0,
                        kind: PrimKind::Bool,
                    });
                }
                Kind::ConStr(id) => {
                    outputs.push(Prim {
                        cst_id: id.0,
                        kind: PrimKind::ConStr,
                    });
                }
                Kind::Void(_) => {}
                Kind::Alias(alias) => {
                    let Some(alias) = lookup_alias.get(alias.name.inner.as_str()) else {
                        return Err(());
                    };
                    let res = helper(*alias, lookup_alias, remain - 1)?;
                    outputs.extend(res);
                }
            }
        }

        Ok(outputs)
    }

    // resolve type alias
    for decl in n {
        match decl {
            Decl::Alias(decl_alias) => {
                let res = helper(decl_alias, &lookup_alias, max_path);
                match res {
                    Ok(prims) => {
                        let alias = TLTAlias { prims };
                        tlt.alias.insert(&decl_alias.name.inner, alias);
                    }
                    Err(_) => {
                        tlt.errors.push(CFoodError {
                            message: "Invalid alias declaration".to_owned(),
                            help: Some(
                                "Ensure all aliases exist and avoid cyclic alias chains."
                                    .to_owned(),
                            ),
                            labels: vec![CFoodErrorLabel {
                                cst_id: decl_alias.id,
                                label: Some("cannot resolve alias types here".to_owned()),
                            }],
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
pub fn check_decl_var<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b DeclVar) -> Result<()> {
    let Some(mut outputs) = tlt.normaliaze_kind(&n.ty) else {
        tlt.errors.push(CFoodError {
            message: "Cannot resolve declared type".to_owned(),
            help: Some("Use a built-in type or declare the alias before this variable.".to_owned()),
            labels: vec![CFoodErrorLabel {
                cst_id: n.ty.mark(),
                label: Some("type cannot be resolved here".to_owned()),
            }],
        });
        tlt.type_store.unknown(n.id);
        return Ok(());
    };
    outputs.reverse();

    let id = tlt.type_store.c_type(CType {
        cst_id: n.id,
        outputs,
        ..Default::default()
    });

    if let Some(expr) = &n.init {
        tlt.check_expr(expr)?;
        let expr_id = tlt.type_store.get_type_id(expr.mark()).unwrap();
        if !tlt.type_store.is_eq(id, expr_id) {
            let declared = tlt.type_store.get(id).to_string();
            let actual = tlt.type_store.get(expr_id).to_string();
            tlt.errors.push(CFoodError {
                message: "Type mismatch in variable initialization".to_owned(),
                help: Some(
                    "Change the initializer or declared type so both types match.".to_owned(),
                ),
                labels: vec![
                    CFoodErrorLabel {
                        cst_id: n.ty.mark(),
                        label: Some(format!("declared as `{declared}`")),
                    },
                    CFoodErrorLabel {
                        cst_id: expr.mark(),
                        label: Some(format!("initializer has type `{actual}`")),
                    },
                ],
            });
        }
    }
    let block = tlt.block.last_mut().unwrap();
    let prev_declare = block.insert(&n.name.inner, (n.id, id));
    if let Some((cst_id, _)) = prev_declare {
        tlt.errors.push(CFoodError {
            message: "Redeclaration of variable".to_owned(),
            labels: vec![
                CFoodErrorLabel {
                    cst_id: cst_id,
                    label: Some(format!("first variable declared here")),
                },
                CFoodErrorLabel {
                    cst_id: n.id,
                    label: Some(format!("redeclared here")),
                },
            ],
            ..Default::default()
        });
    }

    Ok(())
}

pub fn hoist_decl_func<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b DeclFunc) -> Result<()> {
    let mut params = vec![];
    for param in &n.params {
        let Some(mut outputs) = tlt.normaliaze_kind(&param.ty) else {
            tlt.errors.push(CFoodError {
                message: "Cannot resolve parameter type".to_owned(),
                help: Some(
                    "Use a built-in type or declare the alias before this function.".to_owned(),
                ),
                labels: vec![CFoodErrorLabel {
                    cst_id: param.ty.mark(),
                    label: Some("parameter type cannot be resolved here".to_owned()),
                }],
            });
            tlt.type_store.unknown(param.id);
            return Ok(());
        };
        outputs.reverse();

        let id = tlt.type_store.c_type(CType {
            cst_id: param.id,
            outputs,
            ..Default::default()
        });
        params.push(id);
    }

    let Some(mut outputs) = tlt.normaliaze_kind(&n.ret) else {
        tlt.errors.push(CFoodError {
            message: "Cannot resolve return type".to_owned(),
            labels: vec![CFoodErrorLabel {
                cst_id: n.ret.mark(),
                label: Some("return type cannot be resolved here".to_owned()),
            }],
            ..Default::default()
        });
        tlt.type_store.unknown(n.id);
        return Ok(());
    };
    outputs.reverse();

    let ret = tlt.type_store.c_type(CType {
        cst_id: n.ret.mark(),
        outputs,
        ..Default::default()
    });

    let inputs = params
        .into_iter()
        .rev()
        .map(|x| tlt.type_store.get(x))
        .fold(vec![], |acc, x| {
            [acc, x.clone().expect_c_type().outputs].concat()
        });
    let outputs = tlt.type_store.get(ret).clone().expect_c_type().outputs;
    let func = tlt.type_store.c_type(CType {
        cst_id: n.id,
        inputs,
        outputs,
    });
    let prev_declare = tlt
        .block
        .first_mut()
        .unwrap()
        .insert(&n.name.inner, (n.id, func));
    if let Some((cst_id, _)) = prev_declare {
        tlt.errors.push(CFoodError {
            message: "Redeclaration of function".to_owned(),
            labels: vec![
                CFoodErrorLabel {
                    cst_id: cst_id,
                    label: Some(format!("first function declared here")),
                },
                CFoodErrorLabel {
                    cst_id: n.id,
                    label: Some(format!("redeclared here")),
                },
            ],
            ..Default::default()
        });
    }
    Ok(())
}

pub fn check_decl_func<'a, 'b: 'a>(tlt: &mut TLT<'a>, n: &'b DeclFunc) -> Result<()> {
    let params: Vec<_> = n
        .params
        .iter()
        .map(|p| tlt.type_store.get_type_id(p.id).unwrap())
        .collect();

    let ret = tlt.type_store.get_type_id(n.ret.mark()).unwrap();

    tlt.block.push(Default::default());
    let block = tlt.block.last_mut().unwrap();

    for (id, param) in params.into_iter().zip(&n.params) {
        block.insert(&param.name.inner, (param.id, id));
    }
    block.insert("return", (n.ret.mark(), ret));
    tlt.check_stmt_block(&n.block)?;

    tlt.block.pop();
    Ok(())
}
