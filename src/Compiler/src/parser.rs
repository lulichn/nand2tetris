

// class
//   'class'
//   className
//   '{'
//   classVarDec*
//   subroutineDec*
//   '}'

// className
//    identifier

// classVarDec
//    ('static' | 'field')
//    type
//    varName
//    (',' varName)*
//    ';'

// type
//   'int' | 'char' | 'boolean' | className

// varName
//    identifier

// subroutineDec
//    ('constructor' | 'function' | 'method')
//    ('void' | type)
//    subroutineName
//    '('
//    parameterList
//    ')'
//    subroutineBody

// subroutineName
//    identifier

// parameterList
//    (
//        (type varName)
//        (',' type varName)*
//    )?

// subroutineBody
//    '{'
//    varDec* statements
//    '}'

// varDec
//    'var'
//    type
//    varName
//    (',' varName)*
//    ';'

//----------
// 文
//----------

// statements
//    statement*

// statement
//    letStatement | ifStatement | whileStatement | doStatement | returnStatement

// letStatement
//    'let'
//    varName
//    ('[' expression ']')?
//    '='
//    expression
//    ';'

// ifStatement
//    'if'
//    '('
//    expression
//    ')'
//    '{'
//    statements
//    '}'
//    (
//        'else'
//        '{'
//        statements
//        '}'
//     )?

// whileStatement
//    'while'
//    '('
//    expression
//    ')'
//    '{'
//    statements
//    '}'

// doStatement
//    'do'
//    subroutineCall
//    ';'

// returnStatement
//    'return'
//    expression?
//    ';'

//----------
// 式
//----------

// expression
//    term
//    (
//        op
//        term
//     )*

// term
//     integerConstant |
//       stringConstant |
//       keywordConstant |
//       varName |
//       varName
//         '['
//         expression
//         ']' |
//       subroutineCall |
//       '('
//           expression
//       ')' |
//       unaryOp term

// subroutineCall
//    subroutineName
//    '('
//    expressionList
//    ')' |
//      (className | varName)
//      '.'
//      subroutineName
//      '('
//      expressionList
//      ')'

// expressionList
//    (
//        expression
//        (
//             ','
//             expression
//        )*
//    )?

// op
//    '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='

// unaryOp
//    '-' | '~'

// KeywordConstant
//    'true' | 'false' | 'null' | 'this'
