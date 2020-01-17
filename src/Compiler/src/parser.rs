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

    fn compare(&self, to: Vec<Tokens>) -> bool {
        self.compare_pos(to, 0)
    }

    fn compare_pos(&self, to: Vec<Tokens>, pos: usize) -> bool {
        for idx in 0..=to.len() - 1 {
            let from = self.tokens[self.now + idx + pos].clone();
            let toi = to.get(idx).unwrap().clone();

            match (from, toi) {
                (Tokens::Keyword(ref v1), Tokens::Keyword(ref v2)) => {
                    if *v2 != String::default() && *v1 != *v2 {
                        return false;
                    }
                },
                (Tokens::Identifier(ref v1), Tokens::Identifier(ref v2)) => {
                    if *v2 != String::default() && *v1 != *v2 {
                        return false;
                    }
                },
                (Tokens::Symbol(v1), Tokens::Symbol(v2)) => {
                    if v1 != v2 {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        true
    }

    fn next_is_type(&self) -> bool {
        // type
        self.compare(vec![Tokens::Keyword(String::from("int"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("char"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("boolean"))]) ||
            self.compare(vec![Tokens::Identifier(String::default())])
    }

    fn next_is_constant(&self) -> bool {
        match self.tokens[self.now].clone() {
            Tokens::IntegerConstant(_) => true,
            Tokens::StringConstant(_) => true,
            _ => false
        }
    }

    fn next_is_keyword_constant(&self) -> bool {
        self.compare(vec![Tokens::Keyword(String::from("true"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("false"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("null"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("this"))])
    }

    fn next_is_op(&self) -> bool {
        self.compare(vec![Tokens::Symbol('+')]) ||
            self.compare(vec![Tokens::Symbol('-')]) ||
            self.compare(vec![Tokens::Symbol('*')]) ||
            self.compare(vec![Tokens::Symbol('/')]) ||
            self.compare(vec![Tokens::Symbol('&')]) ||
            self.compare(vec![Tokens::Symbol('|')]) ||
            self.compare(vec![Tokens::Symbol('<')]) ||
            self.compare(vec![Tokens::Symbol('>')]) ||
            self.compare(vec![Tokens::Symbol('=')])
    }

    fn next_is_unary_op(&self) -> bool {
        self.compare(vec![Tokens::Symbol('-')]) ||
            self.compare(vec![Tokens::Symbol('~')])
    }

    fn next_is_subroutine_call(&self) -> bool {
        self.compare(vec![Tokens::Identifier(String::new()), Tokens::Symbol('.')])
    }

    fn next_is_var_dec(&self) -> bool {
        self.compare(vec![Tokens::Keyword(String::from("var"))])
    }

    fn next_is_class_var_dec(&self) -> bool {
        self.compare(vec![Tokens::Keyword(String::from("static"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("field"))])
    }

    fn next_is_let_statement(&self) -> bool { self.next_is_statement("let") }
    fn next_is_if_statement(&self) -> bool { self.next_is_statement("if")}
    fn next_is_while_statement(&self) -> bool { self.next_is_statement("while")}
    fn next_is_do_statement(&self) -> bool { self.next_is_statement("do")}
    fn next_is_return_statement(&self) -> bool { self.next_is_statement("return")}

    fn next_is_statement(&self, keyword: &str) -> bool {
        self.compare(vec![Tokens::Keyword(String::from(keyword))])
    }

    fn next_is_subroutine_dec(&self) -> bool {
        self.compare(vec![Tokens::Keyword(String::from("constructor"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("function"))]) ||
            self.compare(vec![Tokens::Keyword(String::from("method"))])
    }
}

impl Parser {
    pub fn new(tokens: &Vec<Tokens>) -> Parser {
        Parser {
            tokens: TokenHolder::new(tokens.to_vec())
        }
    }

    pub fn parse(&mut self) -> Vec<String> {
        let tokens = self.compile_class();
        tokens
    }

    // class
    //     'class'
    //     className
    //     '{'
    //     classVarDec*
    //     subroutineDec*
    //     '}'
    fn compile_class(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("<class>"));

        vec.push(self.tokens.next().unwrap().xml_node());

        vec.push(self.tokens.next().unwrap().xml_node());

        vec.push(self.tokens.next().unwrap().xml_node());

        while self.tokens.next_is_class_var_dec() {
            vec.append(&mut self.compile_class_var_dec());
        }

        while self.tokens.next_is_subroutine_dec() {
            vec.append(&mut self.compile_subroutine_dec());
        }

        vec.push(self.tokens.next().unwrap().xml_node());

        vec.push(String::from("</class>"));

        vec
    }

    // classVarDec
    //     ('static' | 'field')
    //     type
    //     varName
    //     (',' varName)*
    //     ';'
    fn compile_class_var_dec(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("<classVarDec>"));

        vec.push(self.tokens.next().unwrap().xml_node());

        vec.push(self.tokens.next().unwrap().xml_node());

        vec.push(self.tokens.next().unwrap().xml_node());

        loop {
            let next_token = self.tokens.next();
            match next_token {
                Some(Tokens::Symbol(c)) if c == ',' => {
                    vec.push(next_token.unwrap().xml_node());

                    vec.push(self.tokens.next().unwrap().xml_node());
                },
                Some(Tokens::Symbol(c)) if c == ';' => {
                    vec.push(next_token.unwrap().xml_node());
                    break;
                },
                _ => {
                    unreachable!();
                }
            }
        }

        vec.push(String::from("</classVarDec>"));

        vec
    }

    // subroutineDec
    //     ('constructor' | 'function' | 'method')
    //     ('void' | type)
    //     subroutineName
    //     '('
    //     parameterList
    //     ')'
    //     subroutineBody
    fn compile_subroutine_dec(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<subroutineDec>"));

        vec.push(self.tokens.next().unwrap().xml_node()); // ('constructor' | 'function' | 'method')

        vec.push(self.tokens.next().unwrap().xml_node()); // ('void' | type)

        vec.push(self.tokens.next().unwrap().xml_node()); // subroutineName

        vec.push(self.tokens.next().unwrap().xml_node());  // (

        vec.append(&mut self.compile_parameter_list());   // parameterList

        vec.push(self.tokens.next().unwrap().xml_node()); // )

        vec.append(&mut self.compile_subroutine_body());  // subroutineBody

        vec.push(String::from("</subroutineDec>"));

        vec
    }

    // parameterList
    //     (
    //         (type varName)
    //         (',' type varName)*
    //     )?
    fn compile_parameter_list(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<parameterList>"));

        if self.tokens.next_is_type() && self.tokens.compare_pos(vec![Tokens::Identifier(String::default())], 1) {
            vec.push(self.tokens.next().unwrap().xml_node()); // type
            vec.push(self.tokens.next().unwrap().xml_node()); // varName

            while self.tokens.compare(vec![Tokens::Symbol(',')]) {
                vec.push(self.tokens.next().unwrap().xml_node()); // ,
                vec.push(self.tokens.next().unwrap().xml_node()); // type
                vec.push(self.tokens.next().unwrap().xml_node()); // varName
            }
        }

        vec.push(String::from("</parameterList>"));
        vec
    }

    // subroutineBody
    //     '{'
    //     varDec*
    //     statements
    //     '}'
    fn compile_subroutine_body(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<subroutineBody>"));


        vec.push(self.tokens.next().unwrap().xml_node());

        while self.tokens.next_is_var_dec() {
            vec.append(&mut self.compile_var_dec());
        }

        vec.append(&mut self.compile_statements());

        vec.push(self.tokens.next().unwrap().xml_node());

        vec.push(String::from("</subroutineBody>"));

        vec
    }

    // varDec
    //     'var'
    //     type
    //     varName
    //     (',' varName)*
    //     ';'
    fn compile_var_dec(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<varDec>"));

        vec.push(self.tokens.next().unwrap().xml_node());
        vec.push(self.tokens.next().unwrap().xml_node());
        vec.push(self.tokens.next().unwrap().xml_node());

        loop {
            let next_token = self.tokens.next();
            match next_token {
                Some(Tokens::Symbol(c)) if c == ',' => {
                    vec.push(next_token.unwrap().xml_node());

                    vec.push(self.tokens.next().unwrap().xml_node());
                },
                Some(Tokens::Symbol(c)) if c == ';' => {
                    vec.push(next_token.unwrap().xml_node());
                    break;
                },
                _ => {
                    unreachable!();
                }
            }
        }

        vec.push(String::from("</varDec>"));

        vec
    }

    // statements
    //     statement*

    fn compile_statements(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<statements>"));

        // loop
        let mut has_statement = true;
        while has_statement {
            if self.tokens.next_is_let_statement() {
                vec.append(&mut self.compile_let_statement());
                continue;
            }
            if self.tokens.next_is_if_statement() {
                vec.append(&mut self.compile_if_statement());
                continue;
            }
            if self.tokens.next_is_while_statement() {
                vec.append(&mut self.compile_while_statement());
                continue;
            }
            if self.tokens.next_is_do_statement() {
                vec.append(&mut self.compile_do_statement());
                continue;
            }
            if self.tokens.next_is_return_statement() {
                vec.append(&mut self.compile_return_statement());
                continue;
            }

            has_statement = false;
        }

        vec.push(String::from("</statements>"));

        vec
    }

    // letStatement
    //     'let'
    //     varName
    //     ('[' expression ']')?
    //     '='
    //     expression
    //     ';'
    fn compile_let_statement(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<letStatement>"));
        vec.push(self.tokens.next().unwrap().xml_node());  // let
        vec.push(self.tokens.next().unwrap().xml_node());  // varName

        if self.tokens.compare(vec![Tokens::Symbol('[')]) {
            vec.push(self.tokens.next().unwrap().xml_node()); // [
            vec.append(&mut self.compile_expression());       // expression
            vec.push(self.tokens.next().unwrap().xml_node()); // ]
        }

        vec.push(self.tokens.next().unwrap().xml_node());  // =
        vec.append(&mut self.compile_expression());       // expression
        vec.push(self.tokens.next().unwrap().xml_node());  // ;

        vec.push(String::from("</letStatement>"));

        vec
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
    fn compile_if_statement(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<ifStatement>"));
        vec.push(self.tokens.next().unwrap().xml_node());
        vec.push(self.tokens.next().unwrap().xml_node());
        vec.append(&mut self.compile_expression());
        vec.push(self.tokens.next().unwrap().xml_node());
        vec.push(self.tokens.next().unwrap().xml_node());
        vec.append(&mut self.compile_statements());
        vec.push(self.tokens.next().unwrap().xml_node());

        if self.tokens.compare(vec![Tokens::Keyword(String::from("else"))]) {
            vec.push(self.tokens.next().unwrap().xml_node());
            vec.push(self.tokens.next().unwrap().xml_node());
            vec.append(&mut self.compile_statements());

            vec.push(self.tokens.next().unwrap().xml_node());
        }


        vec.push(String::from("</ifStatement>"));

        vec
    }

    // whileStatement
    //     'while'
    //     '('
    //     expression
    //     ')'
    //     '{'
    //     statements
    //     '}'
    fn compile_while_statement(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<whileStatement>"));
        vec.push(self.tokens.next().unwrap().xml_node()); // while
        vec.push(self.tokens.next().unwrap().xml_node()); // '('
        vec.append(&mut self.compile_expression());       // expression
        vec.push(self.tokens.next().unwrap().xml_node()); // ')'
        vec.push(self.tokens.next().unwrap().xml_node()); // '{'
        vec.append(&mut self.compile_statements());       // statements
        vec.push(self.tokens.next().unwrap().xml_node()); // '}'
        vec.push(String::from("</whileStatement>"));

        vec
    }

    // doStatement
    //     'do'
    //     subroutineCall
    //     ';'
    fn compile_do_statement(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.push(String::from("<doStatement>"));

        vec.push(self.tokens.next().unwrap().xml_node()); // do
        vec.append(&mut self.compile_subroutine_call());  // subroutineCall
        vec.push(self.tokens.next().unwrap().xml_node()); // ;

        vec.push(String::from("</doStatement>"));
        vec
    }

    // returnStatement
    //     'return'
    //     expression?
    //     ';'
    fn compile_return_statement(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("<returnStatement>"));

        vec.push(self.tokens.next().unwrap().xml_node()); // return

        if !self.tokens.compare(vec![Tokens::Symbol(';')]) {
            vec.append(&mut self.compile_expression());  // expression
        }

        vec.push(self.tokens.next().unwrap().xml_node()); // ;

        vec.push(String::from("</returnStatement>"));
        vec
    }

    // expression
    //     term
    //     (
    //         op
    //         term
    //     )*
    fn compile_expression(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        let mut term = self.compile_term();
        if term.is_empty() {
            return term;
        }

        vec.push(String::from("<expression>"));
        vec.append(&mut term);

        while self.tokens.next_is_op() {
            vec.push(self.tokens.next().unwrap().xml_node()); // op
            vec.append(&mut self.compile_term()); // term
        }

        vec.push(String::from("</expression>"));

        vec
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
    fn compile_term(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("<term>"));

        // integerConstant || stringConstant || keywordConstant
        if self.tokens.next_is_constant() || self.tokens.next_is_keyword_constant() {
            vec.push(self.tokens.next().unwrap().xml_node()); //
            vec.push(String::from("</term>"));
            return vec;
        }

        // varName[]
        if self.tokens.compare(vec![Tokens::Identifier(String::default()), Tokens::Symbol('[')]) {
            vec.push(self.tokens.next().unwrap().xml_node()); // varName
            vec.push(self.tokens.next().unwrap().xml_node()); // '['
            vec.append(&mut self.compile_expression());
            vec.push(self.tokens.next().unwrap().xml_node()); // ']'
            vec.push(String::from("</term>"));
            return vec;
        }

        // subroutineCall
        if self.tokens.next_is_subroutine_call() {
            vec.append(&mut self.compile_subroutine_call());
            vec.push(String::from("</term>"));
            return vec;
        }

        // ( expression )
        if self.tokens.compare(vec![Tokens::Symbol('(')]) {
            vec.push(self.tokens.next().unwrap().xml_node()); // '('
            vec.append(&mut self.compile_expression());
            vec.push(self.tokens.next().unwrap().xml_node()); // ')'
            vec.push(String::from("</term>"));
            return vec;
        }

        // unaryOp term
        if self.tokens.next_is_unary_op() {
            vec.push(self.tokens.next().unwrap().xml_node()); // unaryOp
            vec.append(&mut self.compile_term()); // term
            vec.push(String::from("</term>"));
            return vec;
        }

        // varName
        if self.tokens.compare(vec![Tokens::Identifier(String::default())]) {
            vec.push(self.tokens.next().unwrap().xml_node()); // varName
            vec.push(String::from("</term>"));
            return vec;
        }

        Vec::new()
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
    fn compile_subroutine_call(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();


        if self.tokens.compare(vec![Tokens::Identifier(String::new()), Tokens::Symbol('(')]) {
            vec.push(self.tokens.next().unwrap().xml_node()); // subroutineName
            vec.push(self.tokens.next().unwrap().xml_node()); // (
            vec.append(&mut self.compile_expression_list());  //expressionList
            vec.push(self.tokens.next().unwrap().xml_node()); // )
        } else {
            vec.push(self.tokens.next().unwrap().xml_node()); // (className | varName)
            vec.push(self.tokens.next().unwrap().xml_node()); // .
            vec.push(self.tokens.next().unwrap().xml_node()); // subroutineName
            vec.push(self.tokens.next().unwrap().xml_node()); // (
            vec.append(&mut self.compile_expression_list());  // expressionList
            vec.push(self.tokens.next().unwrap().xml_node());  // )
        }
        vec
    }

    // expressionList
    //     (
    //         expression
    //         (
    //              ','
    //              expression
    //         )*
    //     )?
    fn compile_expression_list(&mut self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(String::from("<expressionList>"));

        vec.append(&mut self.compile_expression());  // expression

        while self.tokens.compare(vec![Tokens::Symbol(',')]) {
            vec.push(self.tokens.next().unwrap().xml_node()); // ,
            vec.append(&mut self.compile_expression());
        }

        vec.push(String::from("</expressionList>"));

        vec
    }
}
