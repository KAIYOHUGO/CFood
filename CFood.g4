grammar CFood;

options {
    language = 'Rust';
}

KW_while: 'while';
KW_for: 'for';
KW_if: 'if';
KW_else: 'else';
KW_return: 'return';

KW_type: 'type';
KW_let: 'let';
KW_as: 'as';


TY_int: 'int';
TY_float: 'float';
TY_str: 'str';
TY_void: 'void';
TY_bool: 'bool';


MAGIC_printf: 'printf';
MAGIC_scanf: 'scanf';
MAGIC_new: 'new';

PAREN_L: '(';
PAREN_R: ')';

BRACE_L: '{';
BRACE_R: '}';

NE: '!=';
EQ: '==';
LT: '<';
GT: '>';
LE: '<=';
GE: '>=';

NOT: '!';
AND: '&&';
OR: '||';

PLUS: '+';
SUB: '-';
MOD: '%';
MUL: '*';
DIV: '/';
PEO: '##';

ASSIGN: '=';
COMMA: ',';
REFER: '&';
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

decl: var_decl SEMICOLON
    | ty_decl
    | fn_decl;
var_decl
    : var_decl_ty var_decl_init;
    
var_decl_ty
    : ty_kind IDENT;

var_decl_init
    : ASSIGN expr
    | ;

fn_decl
    : ty_kind IDENT params block;

ty_decl:
    KW_type TYPE ASSIGN PAREN_L tys PAREN_R SEMICOLON;

params
    : PAREN_L param_list PAREN_R
    | PAREN_L TY_void    PAREN_R
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

// yeah type theory naming let's gooooooo
ty_kind
    : TY_int   # ty_kind_ty
    | TY_float # ty_kind_ty
    | TY_str   # ty_kind_ty
    | TY_void  # ty_kind_ty
    | TY_bool  # ty_kind_ty
    | TYPE     # ty_kind_type
    ;

block
    : BRACE_L stmts BRACE_R;
stmts: stmt stmts | ;
stmt:
    branch_stmt
    | iter_stmt
    | for_stmt
    | block
    | var_decl SEMICOLON
    | expr_stmt SEMICOLON
    | let_stmt SEMICOLON
    | return_stmt SEMICOLON
    | SEMICOLON
    ;
expr_stmt:
    expr;

branch_stmt
    : KW_if PAREN_L expr PAREN_R then_branch=stmt
    | KW_if PAREN_L expr PAREN_R then_branch=stmt KW_else else_branch=stmt;

iter_stmt
    : KW_while PAREN_L expr PAREN_R stmt;

inline_stmts
    : inline_stmt
    | inline_stmt COMMA inline_stmts;

inline_stmt
    : var_decl
    | expr_stmt
    | let_stmt
    | return_stmt
    | ;

for_stmt
    : KW_for PAREN_L init=inline_stmts SEMICOLON cond=expr SEMICOLON mutate=inline_stmts PAREN_R stmt;

return_stmt
    : KW_return
    | KW_return expr;

let_stmt
    : KW_let IDENT ASSIGN expr;

var
    : IDENT;

refer
    : REFER IDENT;

expr
    : expr_assign;

expr_assign
    : expr_logic # expr_assign_pass
    | lhs=var ASSIGN rhs=expr_assign # expr_assign_use
    ;

expr_logic
    : expr_cmp # expr_logic_pass
    | lhs=expr_cmp logic_preced_op rhs=expr_logic # expr_logic_use
    ;

expr_cmp
    : expr_add # expr_cmp_pass
    | lhs=expr_add cmp_preced_op rhs=expr_cmp # expr_cmp_use
    ;

expr_add
    : expr_mul # expr_add_pass
    | lhs=expr_mul add_preced_op rhs=expr_add # expr_add_use
    ;

expr_mul
    : expr_cast # expr_mul_pass
    | lhs=expr_cast mul_preced_op rhs=expr_mul # expr_mul_use
    ;

expr_cast
    : expr_unary # expr_cast_pass
    | lhs=expr_unary KW_as rhs=ty_kind # expr_cast_use
    | lhs=expr_unary KW_as REFER rhs=ty_kind # expr_cast_refer_use
    ;

expr_unary
    : expr_magic # expr_unary_pass
    | unary_preced_op rhs=expr_unary # expr_unary_use
    ;

expr_magic
    : expr_call # expr_magic_pass
    | lhs=magic rhs=expr_magic # expr_magic_use
    ;

expr_call
    : atom # expr_call_pass
    | lhs=atom rhs=expr_call # expr_call_use
    ;

atom
    : apply_list # atom_apply_list
    | var        # atom_var
    | refer      # atom_refer
    | lit        # atom_lit
    ;

magic
    : MAGIC_printf
    | MAGIC_scanf
    | MAGIC_new
    ;

// low to high
logic_preced_op
    : AND
    | OR;

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

unary_preced_op
    : PLUS
    | SUB
    | NOT;

apply_list
    : PAREN_L PAREN_R
    | PAREN_L args PAREN_R;

args
    : expr COMMA args
    | expr
    |;
