grammar CFood;

options {
    language = 'Rust';
}

@header {
	use crate::ty::*;
	use crate::tlt::*;
}

@fields {
	pub tlt: TLT,
}
@init {
    tlt: Default::default(),
}

KW_while: 'while';
KW_if: 'if';
KW_else: 'else';
KW_return: 'return';

KW_type: 'type';
KW_let: 'let';


TY_int: 'int';
TY_float: 'float';
TY_void: 'void';

ARROW: '->';
PAREN_L: '(';
PAREN_R: ')';

BRACE_L: '{';
BRACE_R: '}';

BRACKET_L: '[';
BRACKET_R: ']';

NE: '!=';
EQ: '==';
LT: '<';
GT: '>';
LE: '<=';
GE: '>=';

PLUS: '+';
SUB: '-';
MOD: '%';
MUL: '*';
DIV: '/';

ASSIGN: '=';
COMMA: ',';
SEMICOLON: ';';

TYPE: [A-Z] [a-zA-Z0-9_]*;
IDENT: [a-z_] [a-zA-Z0-9_]*;

INT: [0-9]+;
FLOAT: [0-9]+ '.' [0-9]+;

LINE_COMMENT: '//' .*? '\n' -> skip;
COMMENT: '/*' .*? '*/' -> skip;
WS: [ \t\r\n]+ -> skip;


file: decls;
decls: decl decls | ;

decl: var_decl | ty_decl | fn_decl;
var_decl returns [ty_id: TyId]
    : var_decl_ty
    {
        $ty_id = *$var_decl_ty.ty_id;
    }
    var_decl_init[$ty_id] SEMICOLON;
    
var_decl_ty returns [ty_id: TyId]
    : ty IDENT
    {
        let name = $IDENT.text;
        let ty_raw = $ty.ty_raw;
        let line = $IDENT.line;

        $ty_id = recog.tlt.new_var_raw(
            name,
            ty_raw,
            line
        );
    }
    | ty_kind IDENT BRACKET_L number BRACKET_R
    {
        let name = $IDENT.text;
        let kind = $ty_kind.kind;
        let num = $number.num;
        let line = $IDENT.line;

        $ty_id = recog.tlt.new_var_arr(
            name,
            kind,
            num,
            line,
        );
    };
var_decl_init[ty_id: TyId]
    : ASSIGN expr
    {
        let bound = $ty_id;
        let expr = *$expr.ty_id;
        let line = $ASSIGN.line;
        recog.tlt.assert_ty_id(bound, expr, line);
    }
    | ;
fn_decl returns [ty_id: TyId]
    : ty IDENT {
        recog.tlt.enter_block();
    } params {
        let name = $IDENT.text;
        let ty_raw = $ty.ty_raw;
        let ty_ids = $params.ty_ids;
        let line = $IDENT.line;
        $ty_id = recog.tlt.new_fn(name, ty_raw, ty_ids, line);
    } block {
        recog.tlt.exit_block();
    };

ty_decl:
    KW_type TYPE ASSIGN PAREN_L tys PAREN_R SEMICOLON
    {
        let name = $TYPE.text.to_owned();
        let kinds = $tys.kinds;
        let line = $TYPE.line;
        recog.tlt.new_alias(
            name,
            kinds,
            line,
        );
    };

params returns [ty_ids: Vec<TyId>]
    : PAREN_L param_list PAREN_R
    {
        $ty_ids = $param_list.ty_ids;
    }
    | PAREN_L TY_void PAREN_R {$ty_ids = vec![];}
    | PAREN_L PAREN_R {$ty_ids = vec![];};
param_list returns [ty_ids: Vec<TyId>]
    : param COMMA param_list
    {
        $ty_ids = [vec![*$param.ty_id], $param_list.ty_ids.to_owned()].concat();
    }
    | param
    {
        $ty_ids = vec![*$param.ty_id];
    };

param returns [ty_id: TyId]
    : var_decl_ty {$ty_id = $var_decl_ty.ty_id;};

number returns [num: Number]
    : INT {$num = Number::parse_int($INT.text);}
    | FLOAT {$num = Number::parse_float($FLOAT.text);};

tys returns [kinds: Vec<TyKind>]
    : ty_kind COMMA tys
    {
        $kinds = [vec![$ty_kind.kind.clone()], $tys.kinds.clone()].concat();
    }
    | ty_kind
    {
        $kinds = vec![$ty_kind.kind.clone()];
    };
ty returns [ty_raw: TyRaw]
    : ty_kind {$ty_raw = TyRaw::Kind($ty_kind.kind.clone());}
    | ty_kind ARROW ty {$ty_raw = TyRaw::Arrow(
            $ty_kind.kind.clone(),
            Box::new($ty.ty_raw.clone())
        );
    };
// yeah type theory naming let's gooooooo
ty_kind returns [kind: TyKind]
    : TY_int {$kind = TyKind::Primitive(Primitive::Int);}
    | TY_float {$kind = TyKind::Primitive(Primitive::Float);}
    | TY_void {$kind = TyKind::Primitive(Primitive::Void);}
    | TYPE {$kind = TyKind::Type($TYPE.text.to_string());};

block
    : BRACE_L {
        recog.tlt.enter_block();
    } stmts {
        recog.tlt.exit_block();
    } BRACE_R;
stmts: stmt stmts | ;
stmt: var_decl | let_stmt | expr SEMICOLON
    | branch_stmt | iter_stmt | return_stmt | block;
may_empty_stmt: stmt | SEMICOLON;
branch_stmt
    : KW_if PAREN_L expr PAREN_R may_empty_stmt
    {
        let bound = recog.tlt.new_ty(Ty::bool());
        let expr = *$expr.ty_id;
        let line = $KW_if.line;
        recog.tlt.assert_ty_id(bound, expr, line);
    }
    | KW_if PAREN_L expr PAREN_R may_empty_stmt KW_else may_empty_stmt
    {
        let bound = recog.tlt.new_ty(Ty::bool());
        let expr = *$expr.ty_id;
        let line = $KW_if.line;
        recog.tlt.assert_ty_id(bound, expr, line);
    };

iter_stmt
    : KW_while PAREN_L expr PAREN_R may_empty_stmt
    {
        let bound = recog.tlt.new_ty(Ty::bool());
        let expr = *$expr.ty_id;
        let line = $KW_while.line;
        recog.tlt.assert_ty_id(bound, expr, line);
    };
return_stmt
    : KW_return SEMICOLON
    {
        let line = $KW_return.line;
        let bound = recog.tlt.var("return", line);
        let expr = recog.tlt.new_ty(Ty::void());
        recog.tlt.assert_ty_id(bound, expr, line);
    }
    | KW_return expr SEMICOLON
    {
        let line = $KW_return.line;
        let bound = recog.tlt.var("return", line);
        let expr = *$expr.ty_id;
        recog.tlt.assert_ty_id(bound, expr, line);
    };

let_stmt
    : KW_let IDENT ASSIGN expr SEMICOLON
    {
        let name = $IDENT.text;
        let expr = *$expr.ty_id;
        let line = $KW_let.line;

        recog.tlt.new_let(
            name,
            expr,
            line
        );
        
    };

expr returns [ty_id: TyId]
    : assign_expr
    {
        $ty_id = *$assign_expr.ty_id;
    }
    | calc_expr
    {
        $ty_id = *$calc_expr.ty_id;
    };
assign_expr returns [ty_id: TyId]
    : var ASSIGN expr
    {
        $ty_id = *$var.ty_id;

        let bound = *$var.ty_id;
        let expr = *$expr.ty_id;
        let line = $ASSIGN.line;
        recog.tlt.assert_ty_id(bound, expr, line);
    };

var returns [ty_id: TyId]
    : IDENT
    {
        let name = $IDENT.text;
        let line = $IDENT.line;
        $ty_id = recog.tlt.var(name, line);
    }
    | IDENT BRACKET_L expr BRACKET_R
    {
        let name = $IDENT.text;
        let line = $IDENT.line;
        $ty_id = recog.tlt.var_arr(name, line);

        let bound = recog.tlt.new_ty(Ty::int());
        let expr = *$expr.ty_id;
        recog.tlt.assert_ty_id(bound, expr, line);
    };
calc_expr returns [ty_id: TyId]
    : lhs=call_preced_expr cmp_preced_op rhs=call_preced_expr
    {
        let lhs= *$lhs.ty_id;
        let rhs = *$rhs.ty_id;
        let line = *$cmp_preced_op.line;
        $ty_id = recog.tlt.cmp_op(lhs, rhs, line);
    }
    | call_preced_expr { $ty_id = $call_preced_expr.ty_id; };
call_preced_expr returns [ty_id: TyId]
    : add_preced_expr call_preced_expr
    {
        let expr = *$add_preced_expr.ty_id;
        let call = *$call_preced_expr.ty_id;
        let line = recog.get_current_context().start().get_line();
        $ty_id = recog.tlt.apply(expr, call, line);
    }
    | add_preced_expr { $ty_id = $add_preced_expr.ty_id; };

add_preced_expr returns [$ty_id: TyId]
    : mul_preced_expr add_preced_op add_preced_expr 
    {
        let lhs = *$mul_preced_expr.ty_id;
        let rhs = *$add_preced_expr.ty_id;
        let line = *$add_preced_op.line;
        $ty_id = recog.tlt.binary_op(lhs, rhs, line);
    }
    | mul_preced_expr { $ty_id = *$mul_preced_expr.ty_id; };

mul_preced_expr returns [ty_id: TyId]
    : atom_preced_expr mul_preced_op mul_preced_expr 
    {
        let lhs = *$atom_preced_expr.ty_id;
        let rhs = *$mul_preced_expr.ty_id;
        let line = *$mul_preced_op.line;
        $ty_id = recog.tlt.binary_op(lhs, rhs, line);
    }
    | atom_preced_expr { $ty_id = *$atom_preced_expr.ty_id; };
atom_preced_expr returns [ty_id: TyId]
    : apply_list {$ty_id = *$apply_list.ty_id; }
    | var { $ty_id = *$var.ty_id; }
    | number
    {
        let num = *$number.num;
        $ty_id = recog.tlt.new_ty_from_number(num);
    };

// low to high
cmp_preced_op returns [line: u32]
    : NE { $line = $NE.line; }
    | EQ { $line = $EQ.line; }
    | LT { $line = $LT.line; }
    | GT { $line = $GT.line; }
    | LE { $line = $LE.line; }
    | GE { $line = $GE.line; };
add_preced_op returns [line: u32]
    : PLUS { $line = $PLUS.line; }
    | SUB { $line = $SUB.line; };
mul_preced_op returns [line: u32]
    : MUL { $line = $MUL.line; }
    | DIV { $line = $DIV.line; }
    | MOD { $line = $MOD.line; };

apply_list returns [ty_id: TyId]
    : PAREN_L args PAREN_R
    {
        $ty_id = *$args.ty_id;
    };
args returns [ty_id: TyId]
    : expr COMMA args
    {
        let expr = *$expr.ty_id;
        let args = *$args.ty_id;
        let line = $COMMA.line;
        $ty_id = recog.tlt.apply(expr, args, line);
    }
    | expr
    {
        $ty_id = *$expr.ty_id;;
    }
    | { $ty_id = recog.tlt.new_ty(Ty::void()); };
