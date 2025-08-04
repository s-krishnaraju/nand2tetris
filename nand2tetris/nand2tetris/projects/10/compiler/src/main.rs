use std::env;
use std::fs;

enum Keyword {
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
            _ => None,
        }
    }
}

enum Symbol {
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
    Slash,
    And,
    Or,
    Less,
    Greater,
    Equal,
    Negation,
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
            b'/' => Some(Symbol::Slash),
            b'&' => Some(Symbol::And),
            b'|' => Some(Symbol::Or),
            b'<' => Some(Symbol::Less),
            b'>' => Some(Symbol::Greater),
            b'=' => Some(Symbol::Equal),
            b'~' => Some(Symbol::Negation),
            _ => None,
        }
    }
}

enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    IntConst(String),
    StringConst(String),
}

struct Lexer {
    input: Vec<u8>,
    start_pos: usize,
    read_pos: usize,
    line_num: u8,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(ch_vec: Vec<u8>) -> Lexer {
        return Lexer {
            input: ch_vec,
            start_pos: 0,
            read_pos: 0,
            line_num: 1,
            tokens: Vec::new(),
        };
    }

    fn peek_next(&self) -> Option<u8> {
        if self.read_pos == self.input.len() {
            return None;
        }

        return Some(self.input[self.read_pos]);
    }

    // this will throw out of bounds
    fn next(&mut self) -> u8 {
        let ch = self.input[self.read_pos];
        self.read_pos += 1;

        return ch;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Can't read file!");

    let ch_vec: Vec<u8> = contents.bytes().collect();
    let mut lexer = Lexer::new(ch_vec);
    tokenize(&mut lexer);
}

fn handle_symbol(symbol: Symbol, lexer: &mut Lexer) {
    match symbol {
        Symbol::Slash => match lexer.next() {
            b'/' => loop {
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
            },
            b'*' => loop {
                match lexer.next() {
                    b'*' if lexer.peek_next() == Some(b'/') => {
                        lexer.next();
                        break;
                    }
                    _ if lexer.peek_next() == None => {
                        panic!("Invalid block comment end");
                    }
                    b'\n' => {
                        lexer.line_num += 1;
                    }
                    _ => {
                        continue;
                    }
                }
            },
            _ => {
                lexer.tokens.push(Token::Symbol(symbol));
            }
        },
        _ => {
            lexer.tokens.push(Token::Symbol(symbol));
        }
    };
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

    lexer.tokens.push(Token::IntConst(num));
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
                panic!("Missing ending string double quote!");
            }
        }
    }

    lexer.tokens.push(Token::StringConst(s));
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
            None => {}
        }
    }

    if let Some(keyword) = Keyword::to_keyword(&s) {
        lexer.tokens.push(Token::Keyword(keyword));
    } else {
        lexer.tokens.push(Token::Identifier(s));
    }
}

// TODO: test this
fn tokenize(lexer: &mut Lexer) {
    loop {
        let ch = lexer.next();

        if let Some(symbol) = Symbol::to_symbol(ch) {
            handle_symbol(symbol, lexer);
            continue;
        }

        // read_pos will be at next token after this
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
                lexer.next();
            }
            b'\n' => {
                lexer.line_num += 1;
                lexer.next();
            }

            // Either keyword or identifier (can't start with digit)
            _ if ch.is_ascii_alphabetic() || ch == b'_' => {
                handle_keyword_or_identifier(ch, lexer);
            }

            _ => {
                panic!("Unknown matched characted");
            }
        }

        // we are not using start pos at all
        lexer.start_pos = lexer.read_pos;
    }
}
