use std::env;
use std::fs;
use std::io::Error;

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
    fn as_str(&self) -> &'static str {
        match self {
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
    IntConst(u16),
    StringConst(String),
}

struct Lexer {
    input: Vec<u8>,
    start_pos: usize,
    read_pos: usize,
    ch: u8,
}

impl Lexer {
    fn new(ch_vec: Vec<u8>) -> Lexer {
        let first_ch = ch_vec[0];
        return Lexer {
            input: ch_vec,
            start_pos: 0,
            read_pos: 0,
            ch: first_ch,
        };
    }

    // this will throw out of bounds
    fn next(&mut self) -> u8 {
        let ch = self.ch;
        self.read_pos += 1;
        self.ch = self.input[self.read_pos];

        return ch;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut tokens: Vec<Token> = Vec::new();

    let contents = fs::read_to_string(file_path).expect("Can't read file!");

    tokenize(&contents, &mut tokens);
}

fn is_digit(ch: u8) -> bool {
    match ch {
        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => true,
        _ => false,
    }
}

fn handle_symbol(symbol: Symbol, lexer: &mut Lexer) -> Option<Token> {
    match symbol {
        Symbol::Slash => {
            lexer.next();
            match lexer.ch {
                b'/' => {
                    while lexer.next() != b'\n' {
                        continue;
                    }
                }
                b'*' => {
                    while lexer.next() != b'*' {
                        continue;
                    }

                    if lexer.next() != b'/' {
                        panic!("Invalid end comment");
                    }
                }
                _ => {
                    return Some(Token::Symbol(symbol));
                }
            }
        }
        _ => {
            return Some(Token::Symbol(symbol));
        }
    };
    return None;
}

fn tokenize(contents: &str, tokens: &mut Vec<Token>) {
    let mut line_num = 0;
    let ch_vec: Vec<u8> = contents.bytes().collect();

    let mut lexer = Lexer::new(ch_vec);

    loop {
        let ch = lexer.next();

        if let Some(symbol) = Symbol::to_symbol(ch) {
            let result = handle_symbol(symbol, &mut lexer);
            if let Some(token) = result {
                tokens.push(token);
            }
        } else if is_digit(ch) {
        }
    }
}
