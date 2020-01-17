grammar Jack;

@header {
    package net.lulichn.n2t.compiler;
}

klass
    : 'class' className '{' classVarDec* subroutineDec* '}'
    ;

classVarDec
    : varType type varList ';'
    ;

varType
    : 'static' | 'field'
    ;

varList
    : varName (',' varName)*
    ;


type
    : 'int' | 'char' | 'boolean' | className
    ;

subroutineDec
    : subroutineKind returnType subroutineName '(' parameterList ')' subroutineBody
    ;

subroutineKind
    : 'constructor' | 'function' | 'method'
    ;

returnType
    : 'void' | type
    ;

parameterList
    : (typedVar (',' typedVar)*)?
    ;

typedVar
    : type varName
    ;

subroutineBody
    : '{' varDec* statements '}'
    ;

varDec
    : 'var' type varList ';'
    ;

className
    : identifier
    ;

subroutineName
    : identifier
    ;

varName
    : identifier
    ;

statements
    : statement*
    ;

statement
    : letStatement | ifStatement | whileStatement | doStatement | returnStatement
    ;

letStatement
    : 'let' varName arrayIndexing? '=' expression ';'
    ;

arrayIndexing
    : '[' expression ']'
    ;

ifStatement
    : 'if' '(' expression ')' '{' statements '}' elseClause?
    ;

elseClause
    : 'else' '{' statements '}'
    ;

whileStatement
    : 'while' '(' expression ')' '{' statements '}'
    ;

doStatement
    : 'do' subroutineCall ';'
    ;

returnStatement
    : 'return' expression? ';'
    ;

expression
    : term (op term)*
    ;

term
    : integerConstant
    | stringConstant
    | keywordConstant
    | varName
    | varName '[' expression ']'
    | subroutineCall
    | '(' expression ')'
    | unaryOp term
    ;

subroutineCall
    : (qualifier '.')? subroutineName '(' expressionList ')'
    ;

qualifier
    : className | varName
    ;

expressionList
    : (expression (',' expression)* )?
    ;

integerConstant
    : INTEGER
    ;

stringConstant
    : STRING
    ;

identifier
    : ID
    ;
op
    : '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='
    ;

unaryOp
    : '-' | '~'
    ;

keywordConstant
    : 'true' | 'false' | 'null' | 'this'
    ;

STRING       : '"' STRING_CHAR* '"';
INTEGER      : [0-9]+;
ID           : [a-zA-Z_][a-zA-Z0-9_]*;
WS           :  [ \t\r\n\u000C]+ -> skip;
COMMENT      :   '/*' .*? '*/' -> skip;
LINE_COMMENT : '//' ~[\r\n]* -> skip;

fragment STRING_CHAR :   ~["\\] | ESCAPED_CHAR ;
fragment ESCAPED_CHAR : '\\' [btnfr"'\\];

