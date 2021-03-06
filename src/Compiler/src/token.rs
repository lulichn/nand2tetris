use std::error::Error;

static KEYWORDS: &'static [&str] = &[
    "class", "constructor", "function", "method", "field", "static", "var",
    "int", "char", "boolean", "void", "true", "false", "null", "this", "let",
    "do", "if", "else", "while", "return"
];

const SYMBOLS: &'static [char] = &[
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~'
];

fn keyword(str: &str) -> bool {
    KEYWORDS.contains(&str)
}

fn symbol(c: &char) -> bool {
    SYMBOLS.contains(c)
}

pub fn read_file(input: &String) -> Result<Vec<Tokens>, Box<dyn Error>> {
    let mut reader = Reader::new(input);
    let mut stack = Stack::new();

    let mut tokens: Vec<Tokens> = Vec::new();

    loop {
        match reader.read(2) {
            Some(w) if w.as_str() == "//" => {
                reader.skip_newline();
            },
            Some(w) if w.as_str() == "/*" => {
                reader.skip("*/");
            },
            _ => {}
        }

        match reader.next() {
            Some(c) if c.is_newline() => {}
            Some(c) if c.is_whitespace() || symbol(c) => {
                if !stack.is_empty() {
                    tokens.push(stack.to_token());
                    stack.clear();
                }

                if symbol(c) {
                    tokens.push(Tokens::Symbol(*c));
                }
            },
            Some(c) if *c == '"' => {
                let mut w = Vec::new();
                while let Some(c) = reader.next() {
                    if *c == '"' { break }
                    w.push(c.clone());
                }
                tokens.push(Tokens::StringConstant(w.iter().cloned().collect::<String>()));
            },
            Some(c) => {
                stack.push(*c);
            },
            None => break,
        }
    }



    Ok(tokens)
}

trait CharMixin {
    fn is_newline(&self) -> bool;
}

impl CharMixin for char {
    fn is_newline(&self) -> bool {
        match self {
            '\x0a' | '\x0d' => true,
            _ => false,
        }
    }
}

struct Reader {
    index: usize,
    vec: Vec<char>
}

impl Reader {
    fn new(str: &String) -> Reader {
        Reader {
            index: 0,
            vec: str.chars().collect::<Vec<char>>()
        }
    }

    fn next(&mut self) -> Option<&char> {
        self.index;
        if self.index > self.vec.len() {
            return Option::None;
        }
        let c = self.vec.get(self.index);
        self.index += 1;

        c
    }

    fn read(&self, size: usize) -> Option<String> {
        let to = self.index + size;
        if to > self.vec.len() {
            return Option::None;
        }
        Option::Some(self.vec[self.index..to].iter().cloned().collect::<String>())
    }

    fn skip(&mut self, str: &str) {
        while let Some(words) = self.read(str.len()) {
            if words.as_str() == str {
                self.index += str.len();
                break;
            } else {
                self.index += 1;
            }
        }
    }

    fn skip_newline(&mut self) {
        while let Some(c) = self.next() {
            if c.is_newline() {
                self.index -= 1;
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Stack {
    work: Vec<char>
}

impl Stack {
    const fn new() -> Stack {
        Stack {
            work: Vec::new()
        }
    }

    fn clear(&mut self) {
        self.work.clear()
    }

    pub fn push(&mut self, value: char) {
        self.work.push(value)
    }

    fn is_empty(&self) -> bool {
        self.work.len() == 0
    }

    fn to_token(&self) -> Tokens {
        if self.is_keyword() {
            return Tokens::Keyword(self.to_string());
        }

        if self.is_symbol() {
            return Tokens::Symbol(*self.work.get(0).unwrap());
        }

        if self.is_integer_constant() {
            return Tokens::IntegerConstant(self.to_i())
        }

        return Tokens::Identifier(self.to_string())
    }

    fn to_string(&self) -> String {
        self.work.clone().into_iter().collect::<String>()
    }

    fn to_i(&self) -> i32 {
        self.to_string().parse::<i32>().unwrap()
    }

    fn is_keyword(&self) -> bool {
        match self.work.clone().into_iter().collect::<String>().as_str() {
            str => keyword(str)
        }
    }

    fn is_symbol(&self) -> bool {
        match self.work.clone() {
            v if v.len() == 1 => symbol(v.get(0).unwrap()),
            _ => false
        }
    }

    fn is_integer_constant(&self) -> bool {
        let c = self.work.get(0).unwrap();
        ('0'..='9').contains(c)
    }
}

/**
 *
 */
#[derive(Clone,PartialOrd, PartialEq, Debug)]
pub enum Tokens {
    Keyword(String),
    Identifier(String),
    StringConstant(String),
    IntegerConstant(i32),
    Symbol(char),
}

impl Tokens {
    pub fn xml_node(&self) -> String {
        match self {
            Tokens::Keyword(v) => format!("<keyword> {} </keyword>", v),
            Tokens::Identifier(v) => format!("<identifier> {} </identifier>", v),
            Tokens::StringConstant(v) => format!("<stringConstant> {} </stringConstant>", v),
            Tokens::IntegerConstant(v) => format!("<integerConstant> {} </integerConstant>", v),
            Tokens::Symbol(v) => match v {
                '<' => format!("<symbol> &lt; </symbol>"),
                '>' => format!("<symbol> &gt; </symbol>"),
                '&' => format!("<symbol> &amp; </symbol>"),
                c => format!("<symbol> {} </symbol>", c),
            },
        }
    }
}
