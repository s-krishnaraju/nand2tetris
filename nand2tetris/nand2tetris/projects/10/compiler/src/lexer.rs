use core::panic;
use std::collections::VecDeque;
use std::mem;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    start_pos: usize,
    read_pos: usize,
    line_num: usize,
    pub tokens: VecDeque<Token>,
}

impl Lexer {
    pub fn new(ch_vec: Vec<u8>) -> Self {
        return Self {
            input: ch_vec,
            start_pos: 0,
            read_pos: 0,
            line_num: 1,
            tokens: VecDeque::new(),
        };
    }

    pub fn peek_next(&self) -> Option<u8> {
        if self.read_pos == self.input.len() {
            return None;
        }

        return Some(self.input[self.read_pos]);
    }

    // this will throw out of bounds
    pub fn next(&mut self) -> u8 {
        let ch = self.input[self.read_pos];
        self.read_pos += 1;

        return ch;
    }
}

#[derive(Debug)]
pub enum Keyword {
    Class,
    Method,
    Function,
    Constructor,
    Int,
    Boolean,
    Char,
    Void,
    Var,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
    This,
    Static,
    Field,
}

impl Keyword {
    fn to_keyword(s: &str) -> Option<Keyword> {
        match s {
            "class" => Some(Keyword::Class),
            "method" => Some(Keyword::Method),
            "function" => Some(Keyword::Function),
            "constructor" => Some(Keyword::Constructor),
            "int" => Some(Keyword::Int),
            "boolean" => Some(Keyword::Boolean),
            "char" => Some(Keyword::Char),
            "void" => Some(Keyword::Void),
            "var" => Some(Keyword::Var),
            "let" => Some(Keyword::Let),
            "do" => Some(Keyword::Do),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),
            "this" => Some(Keyword::This),
            "static" => Some(Keyword::Static),
            "field" => Some(Keyword::Field),
            _ => None,
        }
    }

    fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<keyword> ");
        let keyword = match self {
            Keyword::Class => "class",
            Keyword::Method => "method",
            Keyword::Function => "function",
            Keyword::Constructor => "constructor",
            Keyword::Int => "int",
            Keyword::Boolean => "boolean",
            Keyword::Char => "char",
            Keyword::Void => "void",
            Keyword::Var => "var",
            Keyword::Let => "let",
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::Return => "return",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null",
            Keyword::This => "this",
            Keyword::Static => "static",
            Keyword::Field => "field",
        };
        xml.push_str(keyword);
        xml.push_str(" </keyword>");
        return xml;
    }
}

#[derive(Debug)]
pub enum Symbol {
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBrack,
    RBrack,
    Dot,
    Semicolon,
    Comma,
    Add,
    Minus,
    Mult,
    Division,
    And,
    Or,
    LessThan,
    GreaterThan,
    Equal,
    Not,
}

impl Symbol {
    fn to_symbol(ch: u8) -> Option<Symbol> {
        match ch {
            b'{' => Some(Symbol::LBrace),
            b'}' => Some(Symbol::RBrace),
            b'(' => Some(Symbol::LParen),
            b')' => Some(Symbol::RParen),
            b'[' => Some(Symbol::LBrack),
            b']' => Some(Symbol::RBrack),
            b'.' => Some(Symbol::Dot),
            b';' => Some(Symbol::Semicolon),
            b',' => Some(Symbol::Comma),
            b'+' => Some(Symbol::Add),
            b'-' => Some(Symbol::Minus),
            b'*' => Some(Symbol::Mult),
            b'/' => Some(Symbol::Division),
            b'&' => Some(Symbol::And),
            b'|' => Some(Symbol::Or),
            b'<' => Some(Symbol::LessThan),
            b'>' => Some(Symbol::GreaterThan),
            b'=' => Some(Symbol::Equal),
            b'~' => Some(Symbol::Not),
            _ => None,
        }
    }
    fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<symbol> ");
        let sym = match self {
            Symbol::LBrace => "{",
            Symbol::RBrace => "}",
            Symbol::LParen => "(",
            Symbol::RParen => ")",
            Symbol::LBrack => "[",
            Symbol::RBrack => "]",
            Symbol::Dot => ".",
            Symbol::Semicolon => ";",
            Symbol::Comma => ",",
            Symbol::Add => "+",
            Symbol::Minus => "-",
            Symbol::Mult => "*",
            Symbol::Division => "/",
            Symbol::And => "&amp;",
            Symbol::Or => "|",
            Symbol::LessThan => "&lt;",
            Symbol::GreaterThan => "&gt;",
            Symbol::Equal => "=",
            Symbol::Not => "~",
        };
        xml.push_str(sym);
        xml.push_str(" </symbol>");
        return xml;
    }
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    IntConst(String),
    StringConst(String),
}

impl Token {
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        match self {
            Self::Keyword(x) => {
                let s = x.to_xml();
                xml.push_str(&s);
            }
            Self::Symbol(x) => {
                let s = x.to_xml();
                xml.push_str(&s);
            }
            Self::Identifier(x) => {
                xml.push_str("<identifier> ");
                xml.push_str(x);
                xml.push_str(" </identifier>");
            }
            Self::IntConst(x) => {
                xml.push_str("<integerConstant> ");
                xml.push_str(x);
                xml.push_str(" </integerConstant>");
            }
            Self::StringConst(x) => {
                xml.push_str("<stringConstant> ");
                xml.push_str(x);
                xml.push_str(" </stringConstant>");
            }
        };
        return xml;
    }
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Keyword(k1), Self::Keyword(k2)) => {
                mem::discriminant(k1) == mem::discriminant(k2)
            }
            (Self::Symbol(s1), Self::Symbol(s2)) => mem::discriminant(s1) == mem::discriminant(s2),
            (t1, t2) => mem::discriminant(t1) == mem::discriminant(t2),
        }
    }
}

pub fn tokenize(lexer: &mut Lexer) {
    loop {
        // Check for EOF
        if let None = lexer.peek_next() {
            return;
        }

        let ch = lexer.next();

        if let Some(symbol) = Symbol::to_symbol(ch) {
            handle_symbol(symbol, lexer);
            continue;
        }

        match ch {
            // Integer const
            b'0'..=b'9' => {
                handle_digit(ch, lexer);
            }
            // String const
            b'"' => {
                handle_string(lexer);
            }
            // Whitespace
            b' ' | b'\t' | b'\r' => {
                continue;
            }
            b'\n' => {
                lexer.line_num += 1;
                continue;
            }
            // Either keyword or identifier (can't start with digit)
            _ if ch.is_ascii_alphabetic() || ch == b'_' => {
                handle_keyword_or_identifier(ch, lexer);
            }
            _ => {
                panic!("Unknown matched character at {}", lexer.line_num);
            }
        }

        // we are not using start pos at all
        lexer.start_pos = lexer.read_pos;
    }
}

fn handle_keyword_or_identifier(ch: u8, lexer: &mut Lexer) {
    let mut s = String::new();
    s.push(ch as char);

    loop {
        match lexer.peek_next() {
            Some(ch) if ch.is_ascii_alphanumeric() || ch == b'_' => {
                s.push(ch as char);
                lexer.next();
            }
            Some(_) => {
                break;
            }
            None => {
                panic!("Reached EOF after identifier/keyword! {}", lexer.line_num);
            }
        }
    }

    if let Some(keyword) = Keyword::to_keyword(&s) {
        lexer.tokens.push_back(Token::Keyword(keyword));
    } else {
        lexer.tokens.push_back(Token::Identifier(s));
    }
}

fn handle_string(lexer: &mut Lexer) {
    let mut s = String::new();
    loop {
        match lexer.peek_next() {
            Some(b'"') => {
                lexer.next();
                break;
            }
            Some(ch) => {
                if ch == b'\n' {
                    lexer.line_num += 1;
                }
                s.push(ch as char);
                lexer.next();
            }
            None => {
                panic!("Missing ending string double quote at {}", lexer.line_num);
            }
        }
    }

    lexer.tokens.push_back(Token::StringConst(s));
}

fn handle_digit(ch: u8, lexer: &mut Lexer) {
    let mut num = String::new();
    num.push(ch as char);

    loop {
        match lexer.peek_next() {
            Some(b'0'..=b'9') => {
                num.push(lexer.next() as char);
                continue;
            }
            _ => {
                break;
            }
        }
    }

    lexer.tokens.push_back(Token::IntConst(num));
}

fn handle_symbol(symbol: Symbol, lexer: &mut Lexer) {
    match symbol {
        // check if it's divison or comment
        Symbol::Division => match lexer.peek_next() {
            // new line comment
            Some(b'/') => {
                // consume '/' and look for newline end
                lexer.next();
                loop {
                    match lexer.peek_next() {
                        Some(b'\n') => {
                            lexer.next();
                            lexer.line_num += 1;
                            break;
                        }
                        Some(_) => {
                            lexer.next();
                        }
                        None => {
                            // we reached EOF
                        }
                    }
                }
            }
            // block comment
            Some(b'*') => {
                // consume '*' and look for */ end
                lexer.next();
                // This will fail w/ /*EOF
                loop {
                    match lexer.peek_next() {
                        Some(b'*') => {
                            lexer.next();
                            if lexer.peek_next() == Some(b'/') {
                                // found */
                                lexer.next();
                                break;
                            }
                        }
                        Some(b'\n') => {
                            lexer.line_num += 1;
                            lexer.next();
                        }
                        Some(_) => {
                            lexer.next();
                        }
                        None => {
                            panic!("Block comment not terminated! at {}", lexer.line_num);
                        }
                    }
                }
            }
            Some(_) => {
                lexer.tokens.push_back(Token::Symbol(Symbol::Division));
            }
            None => {
                panic!("Reached EOF after slash at {}", lexer.line_num);
            }
        },
        _ => {
            lexer.tokens.push_back(Token::Symbol(symbol));
        }
    };
}
