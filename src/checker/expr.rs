use anyhow::Result;

use super::TLT;
use crate::{
    checker::{AType, PrimKind},
    cst::{
        Marked,
        tys::{
            Expr, ExprAssign, ExprBinary, ExprCall, ExprCast, ExprLit, ExprMagic, ExprRefer,
            ExprUnary, ExprVar, Magic, Op, UnaryOp,
        },
    },
    error::*,
};

pub fn check_expr(tlt: &mut TLT, n: &Expr) -> Result<()> {
    match n {
        Expr::Binary(expr_binary) => check_expr_binary(tlt, expr_binary),
        Expr::Unary(expr_unary) => check_expr_unary(tlt, expr_unary),
        Expr::Assign(expr_assign) => check_expr_assign(tlt, expr_assign),
        Expr::Call(expr_call) => check_expr_call(tlt, expr_call),
        Expr::Cast(expr_cast) => check_expr_cast(tlt, expr_cast),
        Expr::Magic(expr_magic) => check_expr_magic(tlt, expr_magic),
        Expr::Lit(lit) => check_lit(tlt, lit),
        Expr::Var(expr_var) => check_expr_var(tlt, expr_var),
        Expr::Refer(expr_refer) => check_expr_refer(tlt, expr_refer),
    }
}

pub fn check_expr_binary(tlt: &mut TLT, n: &ExprBinary) -> Result<()> {
    check_expr(tlt, &n.lhs)?;
    check_expr(tlt, &n.rhs)?;
    let lhs = tlt
        .type_store
        .get(tlt.type_store.get_type_id(n.lhs.mark()).unwrap());
    let rhs = tlt
        .type_store
        .get(tlt.type_store.get_type_id(n.rhs.mark()).unwrap());

    match (&lhs, &rhs) {
        (AType::CType(lhs), AType::CType(rhs)) => {
            if !lhs.inputs.is_empty() || !rhs.inputs.is_empty() {
                tlt.errors.push(CFoodError {
                    message: "Invalid operands for binary operator".to_owned(),
                    help: Some(
                        "Apply functions first so each operand is a concrete value.".to_owned(),
                    ),
                    labels: vec![
                        CFoodErrorLabel {
                            cst_id: n.lhs.mark(),
                            label: Some(format!("left operand has type `{lhs}`")),
                        },
                        CFoodErrorLabel {
                            cst_id: n.rhs.mark(),
                            label: Some(format!("right operand has type `{rhs}`")),
                        },
                    ],
                });

                tlt.type_store.unknown(n.id);
                return Ok(());
            }
            if lhs.outputs.len() != 1 || rhs.outputs.len() != 1 {
                tlt.errors.push(CFoodError {
                    message: "Invalid operands for binary operator".to_owned(),
                    help: Some(
                        "Binary operators require exactly one output value on each side."
                            .to_owned(),
                    ),
                    labels: vec![
                        CFoodErrorLabel {
                            cst_id: n.lhs.mark(),
                            label: Some(format!("left operand has type `{lhs}`")),
                        },
                        CFoodErrorLabel {
                            cst_id: n.rhs.mark(),
                            label: Some(format!("right operand has type `{rhs}`")),
                        },
                    ],
                });

                tlt.type_store.unknown(n.id);
                return Ok(());
            }

            if lhs.outputs[0] != rhs.outputs[0] {
                let lhs_ty = lhs.outputs[0].to_string();
                let rhs_ty = rhs.outputs[0].to_string();
                tlt.errors.push(CFoodError {
                    message: "Binary operands must have the same type".to_owned(),
                    help: Some("Convert one side so both operands use the same type.".to_owned()),
                    labels: vec![
                        CFoodErrorLabel {
                            cst_id: n.lhs.mark(),
                            label: Some(format!("left operand is `{lhs_ty}`")),
                        },
                        CFoodErrorLabel {
                            cst_id: n.rhs.mark(),
                            label: Some(format!("right operand is `{rhs_ty}`")),
                        },
                    ],
                });
                tlt.type_store.unknown(n.id);
                return Ok(());
            }

            let kind = match n.op {
                Op::Add(_) | Op::Sub(_) | Op::Mul(_) | Op::Div(_) | Op::OpMod(_) => {
                    let kind = lhs.outputs[0].kind;

                    if !matches!(kind, PrimKind::Int | PrimKind::Float) {
                        tlt.errors.push(CFoodError {
                            message: "Invalid operands for binary operator".to_owned(),
                            help: Some(
                                "Use int/float operands on both sides of arithmetic operator."
                                    .to_owned(),
                            ),
                            labels: vec![
                                CFoodErrorLabel {
                                    cst_id: n.op.mark(),
                                    label: Some("arthmetic operator is used here".to_owned()),
                                },
                                CFoodErrorLabel {
                                    cst_id: n.lhs.mark(),
                                    label: Some(format!("operand type is `{}`", lhs.outputs[0])),
                                },
                                CFoodErrorLabel {
                                    cst_id: n.rhs.mark(),
                                    label: Some(format!("operand type is `{}`", lhs.outputs[0])),
                                },
                            ],
                        });

                        tlt.type_store.unknown(n.id);
                        return Ok(());
                    }

                    kind
                }

                Op::Ne(_) | Op::Eq(_) | Op::Lt(_) | Op::Gt(_) | Op::Le(_) | Op::Ge(_) => {
                    PrimKind::Bool
                }

                Op::And(_) | Op::Or(_) => {
                    if lhs.outputs[0].kind != PrimKind::Bool {
                        tlt.errors.push(CFoodError {
                            message: "Invalid operands for logical operator".to_owned(),
                            help: Some(
                                "Use bool operands on both sides of `&&` or `||`.".to_owned(),
                            ),
                            labels: vec![
                                CFoodErrorLabel {
                                    cst_id: n.op.mark(),
                                    label: Some("logical operator is used here".to_owned()),
                                },
                                CFoodErrorLabel {
                                    cst_id: n.lhs.mark(),
                                    label: Some(format!("operand type is `{}`", lhs.outputs[0])),
                                },
                                CFoodErrorLabel {
                                    cst_id: n.rhs.mark(),
                                    label: Some(format!("operand type is `{}`", rhs.outputs[0])),
                                },
                            ],
                        });

                        tlt.type_store.unknown(n.id);
                        return Ok(());
                    }

                    PrimKind::Bool
                }

                Op::PEO(_) => {
                    if !lhs.outputs[0].kind.is_float() {
                        tlt.errors.push(CFoodError {
                            message: "Invalid operands for operator ##".to_owned(),
                            help: Some("Use float operands on both sides of `##`.".to_owned()),
                            labels: vec![
                                CFoodErrorLabel {
                                    cst_id: n.op.mark(),
                                    label: Some("`##` is used here".to_owned()),
                                },
                                CFoodErrorLabel {
                                    cst_id: n.lhs.mark(),
                                    label: Some(format!("operand type is `{}`", lhs.outputs[0])),
                                },
                                CFoodErrorLabel {
                                    cst_id: n.rhs.mark(),
                                    label: Some(format!("operand type is `{}`", lhs.outputs[0])),
                                },
                            ],
                        });
                        tlt.type_store.unknown(n.id);
                        return Ok(());
                    }
                    PrimKind::Float
                }
            };

            tlt.type_store.prim(kind, n.id);
        }
        _ => {
            tlt.type_store.unknown(n.id);
        }
    }

    Ok(())
}
pub fn check_expr_unary(tlt: &mut TLT, n: &ExprUnary) -> Result<()> {
    check_expr(tlt, &n.rhs)?;
    let rhs = tlt
        .type_store
        .get(tlt.type_store.get_type_id(n.rhs.mark()).unwrap());

    match rhs {
        AType::CType(rhs) => {
            if !rhs.inputs.is_empty() || rhs.outputs.len() != 1 {
                tlt.errors.push(CFoodError {
                    message: "Invalid operand for unary operator".to_owned(),
                    help: Some(
                        "Unary operators require a concrete expression with one output value."
                            .to_owned(),
                    ),
                    labels: vec![CFoodErrorLabel {
                        cst_id: n.rhs.mark(),
                        label: Some(format!("operand has type `{rhs}`")),
                    }],
                });
                tlt.type_store.unknown(n.id);
                return Ok(());
            }

            let kind = rhs.outputs[0].kind;
            let valid = match n.op {
                UnaryOp::Add(_) | UnaryOp::Sub(_) => matches!(kind, PrimKind::Int | PrimKind::Float),
                UnaryOp::Not(_) => kind == PrimKind::Bool,
            };

            if !valid {
                let op = match n.op {
                    UnaryOp::Add(_) => "+",
                    UnaryOp::Sub(_) => "-",
                    UnaryOp::Not(_) => "!",
                };
                let help = match n.op {
                    UnaryOp::Add(_) | UnaryOp::Sub(_) => {
                        format!("Use an int or float operand with unary `{op}`.")
                    }
                    UnaryOp::Not(_) => "Use a bool operand with unary `!`.".to_owned(),
                };
                tlt.errors.push(CFoodError {
                    message: "Invalid operand for unary operator".to_owned(),
                    help: Some(help),
                    labels: vec![CFoodErrorLabel {
                        cst_id: n.rhs.mark(),
                        label: Some(format!("operand has type `{}`", rhs.outputs[0])),
                    }],
                });
                tlt.type_store.unknown(n.id);
                return Ok(());
            }

            tlt.type_store.prim(kind, n.id);
        }
        _ => {
            tlt.type_store.unknown(n.id);
        }
    }

    Ok(())
}

pub fn check_expr_cast(tlt: &mut TLT, n: &ExprCast) -> Result<()> {
    check_expr(tlt, &n.lhs)?;
    let lhs = tlt
        .type_store
        .get(tlt.type_store.get_type_id(n.lhs.mark()).unwrap());
    let Some(outputs) = tlt.normaliaze_kind(&n.rhs) else {
        tlt.errors.push(CFoodError {
            message: "Cannot resolve cast target type".to_owned(),
            help: Some("Use a built-in type or declare the alias before this cast.".to_owned()),
            labels: vec![CFoodErrorLabel {
                cst_id: n.rhs.mark(),
                label: Some("cast target type cannot be resolved here".to_owned()),
            }],
        });
        tlt.type_store.unknown(n.id);
        return Ok(());
    };

    let can_cast = outputs.len() == 1
        && match (lhs, outputs[0].kind) {
            (AType::CType(ctype), kind) => {
                if ctype.outputs.len() != 1 {
                    false
                } else {
                    match (ctype.outputs[0].kind, kind) {
                        (PrimKind::Int, PrimKind::Float)
                        | (PrimKind::Float, PrimKind::Int)
                        | (PrimKind::Bool, PrimKind::Int)
                        | (PrimKind::Int, PrimKind::Bool) => true,
                        (a, b) => a == b,
                    }
                }
            }
            (AType::Unknown(_), PrimKind::Int) | (AType::Unknown(_), PrimKind::Float) => true,
            _ => false,
        };

    if !can_cast {
        tlt.errors.push(CFoodError {
            message: "Invalid type for casting".to_owned(),
            labels: vec![CFoodErrorLabel {
                cst_id: n.rhs.mark(),
                label: Some(format!("operand has type `{}`", outputs[0])),
            }],
            ..Default::default()
        });
        tlt.type_store.unknown(n.id);
        return Ok(());
    }

    let mut outputs = outputs;
    outputs.reverse();
    tlt.type_store.c_type(crate::checker::CType {
        cst_id: n.id,
        outputs,
        ..Default::default()
    });

    Ok(())
}
pub fn check_expr_assign(tlt: &mut TLT, n: &ExprAssign) -> Result<()> {
    check_expr_var(tlt, &n.var)?;
    check_expr(tlt, &n.rhs)?;
    let var = tlt.type_store.get_type_id(n.var.mark()).unwrap();
    let rhs = tlt.type_store.get_type_id(n.rhs.mark()).unwrap();
    let res = tlt.type_store.is_eq(var, rhs);
    if !res {
        let lhs = tlt.type_store.get(var).to_string();
        let rhs = tlt.type_store.get(rhs).to_string();
        tlt.errors.push(CFoodError {
            message: "Assignment type mismatch".to_owned(),
            labels: vec![
                CFoodErrorLabel {
                    cst_id: n.var.mark(),
                    label: Some(format!("variable has type `{lhs}`")),
                },
                CFoodErrorLabel {
                    cst_id: n.rhs.mark(),
                    label: Some(format!("assigned value has type `{rhs}`")),
                },
            ],
            ..Default::default()
        });
    }

    let mut atype = tlt.type_store.get(var).clone();
    match &mut atype {
        AType::CType(ctype) => ctype.cst_id = n.id,
        AType::Unknown(id) => *id = n.id,
    }
    tlt.type_store.a_type(atype);

    Ok(())
}
pub fn check_expr_call(tlt: &mut TLT, n: &ExprCall) -> Result<()> {
    check_expr(tlt, &n.lhs)?;
    check_expr(tlt, &n.rhs)?;
    let lhs = tlt.type_store.get_type_id(n.lhs.mark()).unwrap();
    let rhs = tlt.type_store.get_type_id(n.rhs.mark()).unwrap();
    let res = tlt.type_store.apply(lhs, rhs, n.id);
    match res {
        Ok(_) => {}
        Err(mut err) => {
            if err.help.is_none() {
                err.help = Some(
                    "Make sure the called value accepts the argument type in order.".to_owned(),
                );
            }
            tlt.errors.push(err);
            tlt.type_store.unknown(n.id);
        }
    }

    Ok(())
}
pub fn check_expr_magic(tlt: &mut TLT, n: &ExprMagic) -> Result<()> {
    check_expr(tlt, &n.rhs)?;
    let rhs = tlt.type_store.get_type_id(n.rhs.mark()).unwrap();
    let rhs = tlt.type_store.get(rhs);
    let is_vaild = rhs.as_c_type().is_some_and(|x| {
        x.inputs.is_empty() && x.outputs.last().is_some_and(|x| x.kind == PrimKind::ConStr)
    });
    if !is_vaild {
        let (message, help) = match n.lhs {
            Magic::Printf(_) => (
                "Invalid argument for printf",
                "Pass a string expression to printf.",
            ),
            Magic::Scanf(_) => (
                "Invalid argument for scanf",
                "Pass a string expression to scanf.",
            ),
        };

        tlt.errors.push(CFoodError {
            message: message.to_owned(),
            help: Some(help.to_owned()),
            labels: vec![CFoodErrorLabel {
                cst_id: n.rhs.mark(),
                label: Some(format!("argument has type `{rhs}`")),
            }],
        });
    }

    tlt.type_store.prim(PrimKind::Int, n.id);

    Ok(())
}
pub fn check_lit(tlt: &mut TLT, n: &ExprLit) -> Result<()> {
    match n {
        ExprLit::Int(token) => tlt.type_store.prim(PrimKind::Int, token.id),
        ExprLit::Float(token) => tlt.type_store.prim(PrimKind::Float, token.id),
        ExprLit::ConStr(token) => tlt.type_store.prim(PrimKind::ConStr, token.id),
    };
    Ok(())
}
pub fn check_expr_var(tlt: &mut TLT, n: &ExprVar) -> Result<()> {
    for scope in tlt.block.iter().rev() {
        if let Some((cst_id, id)) = scope.get(n.name.inner.as_str()) {
            let mut a_type = tlt.type_store.get(*id).clone();
            match &mut a_type {
                AType::CType(ctype) => ctype.cst_id = n.id,
                AType::Unknown(id) => *id = n.id,
            }
            tlt.type_store.a_type(a_type);
            tlt.refer_map.insert(n.id, *cst_id);

            return Ok(());
        }
    }
    tlt.errors.push(CFoodError {
        message: "Name not found".to_owned(),
        help: Some("Declare this variable in scope before using it.".to_owned()),
        labels: vec![CFoodErrorLabel {
            cst_id: n.id,
            label: Some(format!("`{}` is not defined", n.name.inner)),
        }],
    });
    tlt.type_store.unknown(n.id);
    Ok(())
}

pub fn check_expr_refer(tlt: &mut TLT, n: &ExprRefer) -> Result<()> {
    tlt.type_store.prim(PrimKind::Int, n.id);
    for scope in tlt.block.iter().rev() {
        if let Some((cst_id, _)) = scope.get(n.name.inner.as_str()) {
            tlt.refer_map.insert(n.id, *cst_id);

            return Ok(());
        }
    }
    tlt.errors.push(CFoodError {
        message: "Name not found".to_owned(),
        help: Some("Declare this variable in scope before taking its reference.".to_owned()),
        labels: vec![CFoodErrorLabel {
            cst_id: n.id,
            label: Some(format!(
                "cannot reference undefined name `{}`",
                n.name.inner
            )),
        }],
    });
    Ok(())
}
