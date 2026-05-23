grammar CFood;

options {
    language = 'Rust';
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
PEO: '##';

ASSIGN: '=';
COMMA: ',';
SEMICOLON: ';';

TYPE: [A-Z] [a-zA-Z0-9_]*;
IDENT: [a-z_] [a-zA-Z0-9_]*;

INT: [0-9]+;
FLOAT: [0-9]+ '.' [0-9]+;
CONSTR: '"' .*? '"';

LINE_COMMENT: '//' .*? '\n' -> skip;
COMMENT: '/*' .*? '*/' -> skip;
WS: [ \t\r\n]+ -> skip;


file: decls;
decls: decl decls | ;

decl: var_decl | ty_decl | fn_decl;
var_decl
    : var_decl_ty var_decl_init SEMICOLON;
    
var_decl_ty
    : ty IDENT
    | ty IDENT BRACKET_L lit BRACKET_R;

var_decl_init
    : ASSIGN expr
    | ;

fn_decl
    : ty IDENT params block;

ty_decl:
    KW_type TYPE ASSIGN PAREN_L tys PAREN_R SEMICOLON;

params
    : PAREN_L param_list PAREN_R
    | PAREN_L TY_void PAREN_R
    | PAREN_L PAREN_R;
param_list
    : param COMMA param_list
    | param;

param
    : var_decl_ty;

lit
    : INT    # lit_int
    | FLOAT  # lit_float
    | CONSTR # lit_constr
    ;

tys
    : ty_kind COMMA tys
    | ty_kind;
ty
    : ty_kind
    | ty_kind ARROW ty;

// yeah type theory naming let's gooooooo
ty_kind
    : TY_int   # ty_kind_ty
    | TY_float # ty_kind_ty
    | TY_void  # ty_kind_ty
    | TYPE     # ty_kind_type
    ;

block
    : BRACE_L stmts BRACE_R;
stmts: stmt stmts | ;
stmt:
    var_decl
    | let_stmt
    | expr_stmt
    | branch_stmt
    | iter_stmt
    | return_stmt
    | block
    | SEMICOLON
    ;
expr_stmt:
    expr SEMICOLON;

branch_stmt
    : KW_if PAREN_L expr PAREN_R then_branch=stmt
    | KW_if PAREN_L expr PAREN_R then_branch=stmt KW_else else_branch=stmt;

iter_stmt
    : KW_while PAREN_L expr PAREN_R stmt;

return_stmt
    : KW_return SEMICOLON
    | KW_return expr SEMICOLON;

let_stmt
    : KW_let IDENT ASSIGN expr SEMICOLON;

expr
    : assign_expr
    | calc_expr;

assign_expr
    : var ASSIGN expr;

var
    : IDENT
    | IDENT BRACKET_L expr BRACKET_R;

calc_expr
    : lhs=call_preced_expr cmp_preced_op rhs=call_preced_expr # calc_expr_use
    | call_preced_expr                                        # calc_expr_pass
    ;

call_preced_expr
    : add_preced_expr call_preced_expr # call_preced_expr_use
    | add_preced_expr                  # call_preced_expr_pass
    ;

add_preced_expr
    : mul_preced_expr add_preced_op add_preced_expr # add_preced_expr_use
    | mul_preced_expr                               # add_preced_expr_pass
    ;

mul_preced_expr
    : atom_preced_expr mul_preced_op mul_preced_expr # mul_preced_expr_use
    | atom_preced_expr                               # mul_preced_expr_pass
    ;

atom_preced_expr
    : apply_list # atom_preced_expr_apply_list
    | var        # atom_preced_expr_var
    | lit        # atom_preced_expr_lit
    ;

// low to high
cmp_preced_op
    : NE
    | EQ
    | LT
    | GT
    | LE
    | GE;
add_preced_op
    : PLUS
    | SUB;
mul_preced_op
    : MUL
    | DIV
    | MOD
    | PEO;

apply_list
    : PAREN_L args PAREN_R;

args
    : expr COMMA args
    | expr
    |;
