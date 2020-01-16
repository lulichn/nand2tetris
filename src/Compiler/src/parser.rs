use crate::token::{Token, Keyword};

pub struct Parser {
    tokens: Tokens
}

pub struct Tokens {
    tokens: Vec<Box<dyn Token>>,
    now: usize,

}

impl Tokens {
    fn new(tokens: Vec<Box<dyn Token>>) -> Tokens {
        Tokens { tokens, now: 0 }
    }

    fn next(&mut self) -> Option<Box<dyn Token>> {
        self.now += 1;
        if self.now - 1 < self.tokens.len() {
            Some(self.tokens[self.now - 1].box_clone())
        } else {
            None
        }
    }

    fn back(&mut self) -> usize {
        self.now -= 1;
        self.now
    }
}

impl Parser {
    pub fn new(tokens: &Vec<Box<dyn Token>>) -> Parser {
        Parser {
            tokens: Tokens::new(tokens.to_vec())
        }
    }

    pub fn parse(&mut self) {
        self.compile_class();
    }

    // class
    //     'class'
    //     className
    //     '{'
    //     classVarDec*
    //     subroutineDec*
    //     '}'
    fn compile_class(&mut self) {
        println!("======== begin compile_class ========");

        let keyword_class = self.tokens.next();
        println!("Keyword: {}", keyword_class.unwrap().xml_node());

        let class_name = self.tokens.next();
        println!("ClassName: {}", class_name.unwrap().xml_node());

        let symbol_begin = self.tokens.next();
        println!("Symbol {}", symbol_begin.unwrap().xml_node());

        self.compile_class_var_dec();
        self.compile_subroutine_dec();

        let symbol_close = self.tokens.next();
        println!("Symbol {}", symbol_close.unwrap().xml_node());
    }

    // classVarDec
    //     ('static' | 'field')
    //     type
    //     varName
    //     (',' varName)*
    //     ';'
    fn compile_class_var_dec(&mut self) {
        println!("======== begin compile_class_var_dec ========");
    }

    fn compile_subroutine_dec(&mut self) {
        println!("======== begin compile_subroutine_dec ========");
    }
}

// className
//    identifier


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
