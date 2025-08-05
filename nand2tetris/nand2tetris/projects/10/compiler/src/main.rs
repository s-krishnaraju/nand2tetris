#![allow(warnings)]
use core::panic;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::mem;

#[derive(Debug)]
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
    Static,
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
            _ => None,
        }
    }
}

#[derive(Debug)]
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
    Division,
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
            b'/' => Some(Symbol::Division),
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

#[derive(Debug)]
enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    IntConst(String),
    StringConst(String),
}

#[derive(Debug)]
struct Lexer {
    input: Vec<u8>,
    start_pos: usize,
    read_pos: usize,
    line_num: u8,
    tokens: VecDeque<Token>,
}

impl Lexer {
    fn new(ch_vec: Vec<u8>) -> Lexer {
        return Lexer {
            input: ch_vec,
            start_pos: 0,
            read_pos: 0,
            line_num: 1,
            tokens: VecDeque::new(),
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

    fn consume_tok(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn peek_tok(&mut self) -> Option<&Token> {
        self.tokens.front()
    }
}

struct TokenParser {
    tokens: VecDeque<Token>,
}
impl TokenParser {
    fn consume_tok(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn peek_tok(&mut self) -> Option<&Token> {
        self.tokens.front()
    }
}

#[derive(Debug)]
enum Statement {
    If,
    Let,
    While,
    Do,
    Return,
}

#[derive(Debug)]
enum NonTerminalType {
    Class,
    ClassVarDec,
    SubroutineDec,
    ParamList,
    SubroutineBody,
    VarDec,
    Statements,
    Statement(Statement),
    ExpressionList,
    Expression,
    Term,
}

#[derive(Debug)]
enum ProgramElement {
    Terminal(Token),
    NonTerminal(NonTerminalElement),
}

// non-terminal elements have children
// terminal just have a token
#[derive(Debug)]
struct NonTerminalElement {
    nt_type: NonTerminalType,
    body: Vec<ProgramElement>,
}

impl NonTerminalElement {
    fn new(nt_type: NonTerminalType) -> NonTerminalElement {
        return NonTerminalElement {
            nt_type: nt_type,
            body: Vec::new(),
        };
    }
    fn add(&mut self, elem: ProgramElement) {
        self.body.push(elem);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if file_path.contains(".jack") {
        let file_name = file_path.split("/").last().unwrap().replace(".jack", "");
        let contents = fs::read_to_string(file_path).expect("Can't read file!");
        let tree = create_program_tree(&contents);
        dbg!(tree);
    } else {
        let paths = fs::read_dir(file_path).expect("Couldn't read directory");
        for path in paths {
            if let Ok(path) = path {
                if let Ok(file_name) = path.file_name().into_string() {
                    if file_name.contains(".jack") {
                        let file_name = file_name.replace(".jack", "");
                        let contents = fs::read_to_string(path.path()).expect("Can't read file!");
                        let tree = create_program_tree(&contents);
                        dbg!(tree);
                    }
                }
            }
        }
    }
}

fn create_program_tree(contents: &str) -> NonTerminalElement {
    let ch_vec: Vec<u8> = contents.bytes().collect();
    let mut lexer = Lexer::new(ch_vec);
    tokenize(&mut lexer);
    let mut parser = TokenParser {
        tokens: lexer.tokens,
    };

    return parse_class(&mut parser);
}

// implement to_str so we can print token
// use
fn match_tok(parsed_tok: Option<Token>, tok: Token) -> ProgramElement {
    match parsed_tok {
        Some(t) => {
            if mem::discriminant(&t) == mem::discriminant(&tok) {
                return ProgramElement::Terminal(tok);
            } else {
                // parsed_tok doesn't match tok
                // use to_str/display to actually display tok
                panic!("Token doesn't match!");
            }
        }
        None => {
            panic!("Reached end of tokens!");
        }
    };
}

// returns root of program/file
fn parse_class(parser: &mut TokenParser) -> NonTerminalElement {
    let mut class = NonTerminalElement::new(NonTerminalType::Class);
    let class_keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Class));
    class.add(class_keyword);
    let class_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
    class.add(class_name);
    let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
    class.add(r_brace);
    // handle classVarDec
    // handle subroutineDec
    let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
    class.add(l_brace);

    return class;
}

fn parse_expression(parser: &mut TokenParser) -> ProgramElement {
    let mut expression = NonTerminalElement::new(NonTerminalType::Expression);
    return ProgramElement::NonTerminal(expression);
}

// recursive handle multiple statements
fn parse_statements(parser: &mut TokenParser) -> ProgramElement {
    let mut statements = NonTerminalElement {
        nt_type: NonTerminalType::Statements,
        body: Vec::new(),
    };

    match parser.consume_tok() {
        Some(Token::Keyword(Keyword::Let)) => {
            // let varName ?[expression]? = expression ;

            match parser.consume_tok() {
                Some(Token::Identifier(var_name)) => {}
                Some(_) => {
                    panic!("Expected identifier after keyword let")
                }
                None => panic!("End of tokens"),
            }
        }
        Some(Token::Keyword(Keyword::If)) => {
            // if (expression) {statements} ?else{statements}?
        }
        Some(Token::Keyword(Keyword::While)) => {
            // while (expression) {statements}
        }
        Some(Token::Keyword(Keyword::Return)) => {
            // return expression? ;
        }
        Some(Token::Keyword(Keyword::Do)) => {
            // do subroutineCall ;
        }
        Some(Token::Symbol(Symbol::RBrace)) => {
            // end statements
        }
        Some(_) => {
            panic!("Unexpected token for statement");
        }
        None => {
            panic!("End of tokens");
        }
    }

    return ProgramElement::NonTerminal(statements);
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

// TODO: test this
fn tokenize(lexer: &mut Lexer) {
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
