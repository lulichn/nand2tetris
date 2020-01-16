use crate::token::{Tokens};

pub struct Parser {
    tokens: TokenHolder
}

pub struct TokenHolder {
    tokens: Vec<Tokens>,
    now: usize,

}

impl TokenHolder {
    fn new(tokens: Vec<Tokens>) -> TokenHolder {
        TokenHolder { tokens, now: 0 }
    }

    fn next(&mut self) -> Option<Tokens> {
        self.now += 1;
        if self.now - 1 < self.tokens.len() {
            Some(self.tokens[self.now - 1].clone())
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
    pub fn new(tokens: &Vec<Tokens>) -> Parser {
        Parser {
            tokens: TokenHolder::new(tokens.to_vec())
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

        println!("======== end compile_class ========");
    }

    // classVarDec
    //     ('static' | 'field')
    //     type
    //     varName
    //     (',' varName)*
    //     ';'
    fn compile_class_var_dec(&mut self) {
        println!("======== begin compile_class_var_dec ========");
        match self.tokens.next() {
            Some(Tokens::Keyword(v)) if v.as_str() == "static" || v.as_str() == "field" => {
                println!("Keyword: {}", v);
            },
            _ => {
                self.tokens.back();
                println!("======== end compile_class_var_dec ========");
                return;
            }
        }

        let var_type = self.tokens.next();
        println!("Var type: {}", var_type.unwrap().xml_node());

        let var_name = self.tokens.next();
        println!("Var name: {}", var_name.unwrap().xml_node());

        loop {
            match self.tokens.next() {
                Some(Tokens::Symbol(c)) if c == ',' => {
                    println!("Symbol: {}", c);

                    let var_name = self.tokens.next();
                    println!("Var name: {}", var_name.unwrap().xml_node());
                },
                Some(Tokens::Symbol(c)) if c == ';' => {
                    println!("Symbol: {}", c);

                    println!("======== end compile_class_var_dec ========");
                    return;
                },
                _ => {
                    unreachable!();
                }
            }
        }
    }

    // subroutineDec
    //     ('constructor' | 'function' | 'method')
    //     ('void' | type)
    //     subroutineName
    //     '('
    //     parameterList
    //     ')'
    //     subroutineBody
    fn compile_subroutine_dec(&mut self) {
        println!("======== begin compile_subroutine_dec ========");
        match self.tokens.next() {
            Some(Tokens::Keyword(v)) if v.as_str() == "constructor" || v.as_str() == "function" || v.as_str() == "method" => {
                println!("Keyword: {}", v);
            },
            _ => {
                self.tokens.back();
                println!("======== end compile_subroutine_dec ========");
                return;
            }
        }

        let return_type = self.tokens.next();
        println!("Return type: {}", return_type.unwrap().xml_node());

        let subroutine_name = self.tokens.next();
        println!("Subroutine name: {}", subroutine_name.unwrap().xml_node());

        let symbol_begin = self.tokens.next();
        println!("symbol: {}", symbol_begin.unwrap().xml_node());

        self.compile_parameter_list();

        let symbol_close = self.tokens.next();
        println!("symbol: {}", symbol_close.unwrap().xml_node());

        self.compile_subroutine_body();

        println!("======== end compile_subroutine_dec ========");
    }

    // parameterList
    //     (
    //         (type varName)
    //         (',' type varName)*
    //     )?
    fn compile_parameter_list(&mut self) {
        println!("======== begin compile_parameter_list ========");
        println!("======== end compile_parameter_list ========");
    }

    // subroutineBody
    //     '{'
    //     varDec*
    //     statements
    //     '}'
    fn compile_subroutine_body(&mut self) {
        println!("======== begin compile_subroutine_body ========");

        let symbol_begin = self.tokens.next();
        println!("symbol: {}", symbol_begin.unwrap().xml_node());

        // loop
        self.compile_var_dev();

        self.compile_statements();

        let symbol_close = self.tokens.next();
        println!("symbol: {}", symbol_close.unwrap().xml_node());

        println!("======== end compile_subroutine_body ========");
    }

    // varDec
    //     'var'
    //     type
    //     varName
    //     (',' varName)*
    //     ';'
    fn compile_var_dev(&mut self) {
        println!("======== begin compile_var_dev ========");
        println!("======== end compile_var_dev ========");

    }

    // statements
    //     statement*

    fn compile_statements(&mut self) {
        println!("======== begin compile_statements ========");

        // loop
        self.compile_statement();

        println!("======== end compile_statements ========");
    }

    // statement
    //     letStatement | ifStatement | whileStatement | doStatement | returnStatement
    fn compile_statement(&mut self) {
        println!("======== begin compile_statement ========");

        println!("======== end compile_statement ========");
    }

    // letStatement
    //     'let'
    //     varName
    //     ('[' expression ']')?
    //     '='
    //     expression
    //     ';'
    fn compile_let_statement(&self) {
        println!("======== begin compile_let_statement ========");

        println!("======== end compile_let_statement ========");
    }

    // ifStatement
    //     'if'
    //     '('
    //     expression
    //     ')'
    //     '{'
    //     statements
    //     '}'
    //     (
    //         'else'
    //         '{'
    //         statements
    //         '}'
    //      )?
    fn compile_if_statement(&self) {
        println!("======== begin compile_if_statement ========");

        println!("======== end compile_if_statement ========");
    }

    // whileStatement
    //     'while'
    //     '('
    //     expression
    //     ')'
    //     '{'
    //     statements
    //     '}'
    fn compile_while_statement(&self) {
        println!("======== begin compile_while_statement ========");

        println!("======== end compile_while_statement ========");
    }

    // doStatement
    //     'do'
    //     subroutineCall
    //     ';'
    fn compile_do_statement(&self) {
        println!("======== begin compile_do_statement ========");

        println!("======== end compile_do_statement ========");
    }

    // returnStatement
    //     'return'
    //     expression?
    //     ';'
    fn compile_return_statement(&self) {
        println!("======== begin compile_return_statement ========");

        println!("======== end compile_return_statement ========");
    }

    // expression
    //     term
    //     (
    //         op
    //         term
    //     )*
    fn compile_expression(&self) {
        println!("======== begin compile_expression ========");

        println!("======== end compile_expression ========");
    }

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
    fn compile_term(&self) {
        println!("======== begin compile_term ========");

        println!("======== end compile_term ========");
    }

    // subroutineCall
    //     subroutineName
    //     '('
    //     expressionList
    //     ')' |
    //       (className | varName)
    //       '.'
    //       subroutineName
    //       '('
    //       expressionList
    //       ')'
    fn compile_subroutine_all(&self) {
        println!("======== begin compile_subroutine_all ========");

        println!("======== end compile_subroutine_all ========");
    }

    // expressionList
    //     (
    //         expression
    //         (
    //              ','
    //              expression
    //         )*
    //     )?
    fn compile_expression_list(&self) {
        println!("======== begin compile_subroutine_all ========");

        println!("======== end compile_subroutine_all ========");
    }
}


// className
//    identifier

// type
//   'int' | 'char' | 'boolean' | className

// varName
//    identifier

// subroutineName
//    identifier


// op
//    '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='

// unaryOp
//    '-' | '~'

// KeywordConstant
//    'true' | 'false' | 'null' | 'this'
