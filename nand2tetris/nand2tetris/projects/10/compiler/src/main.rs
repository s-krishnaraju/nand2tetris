#![allow(warnings)]
use core::panic;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::mem;
use std::ops::RangeInclusive;

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
    LessThan,
    GreaterThan,
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
            b'<' => Some(Symbol::LessThan),
            b'>' => Some(Symbol::GreaterThan),
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

impl Token {
    fn equals(&self, tok: &Token) -> bool {
        mem::discriminant(self) == mem::discriminant(tok)
    }
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
enum NonTerminalType {
    Class,
    ClassVarDec,
    SubroutineDec,
    ParamList,
    SubroutineBody,
    VarDec,
    Statements,
    IfStatement,
    LetStatement,
    WhileStatement,
    DoStatement,
    ReturnStatement,
    ExpressionList,
    Expression,
    Term,
}

#[derive(Debug)]
enum ProgramElement {
    Terminal(Token),
    NonTerminal(NonTerminalElement),
}

impl ProgramElement {
    fn new(tok: Option<Token>) -> ProgramElement {
        match tok {
            Some(t) => ProgramElement::Terminal(t),
            None => panic!(),
        }
    }
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
    fn add_vec(&mut self, v: &mut Vec<ProgramElement>) {
        self.body.append(v);
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

    return handle_class(&mut parser);
}

// check if correct type of token
fn match_tok(parsed_tok: Option<Token>, tok: Token) -> ProgramElement {
    match parsed_tok {
        Some(t) => {
            if t.equals(&tok) {
                return ProgramElement::Terminal(tok);
            } else {
                panic!("Parsed token doesn't match!");
            }
        }
        None => {
            panic!("EOF!");
        }
    };
}

fn handle_class_var_dec(parser: &mut TokenParser) -> ProgramElement {
    let mut class_var_dec = NonTerminalElement::new(NonTerminalType::ClassVarDec);
    let keyword = ProgramElement::new(parser.consume_tok());
    let var_type = handle_declared_type(parser.consume_tok());
    let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));

    class_var_dec.add(keyword);
    class_var_dec.add(var_type);
    class_var_dec.add(var_name);

    // check for ,var_name*
    loop {
        match parser.peek_tok() {
            Some(Token::Symbol(Symbol::Comma)) => {
                let comma = ProgramElement::new(parser.consume_tok());
                let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
                class_var_dec.add(comma);
                class_var_dec.add(var_name);
            }
            Some(Token::Symbol(Symbol::Semicolon)) => {
                let semicolon = ProgramElement::new(parser.consume_tok());
                class_var_dec.add(semicolon);
                break;
            }
            Some(_) => {
                panic!("Expected , or ;");
            }
            None => {
                panic!("Reached end of file!");
            }
        }
    }

    return ProgramElement::NonTerminal(class_var_dec);
}

fn handle_declared_type(tok: Option<Token>) -> ProgramElement {
    match tok {
        Some(t) => {
            if t.equals(&Token::Keyword(Keyword::Boolean))
                || t.equals(&Token::Keyword(Keyword::Char))
                || t.equals(&Token::Keyword(Keyword::Int))
                || t.equals(&Token::Identifier("".to_string()))
            {
                return ProgramElement::Terminal(t);
            } else {
                panic!("type can only be char, int, boolean or a className")
            }
        }
        None => panic!("Reached end of file!"),
    }
}

fn handle_class_var_decs(parser: &mut TokenParser) -> Vec<ProgramElement> {
    let mut decs: Vec<ProgramElement> = Vec::new();
    loop {
        match parser.peek_tok() {
            Some(t)
                if t.equals(&Token::Keyword(Keyword::Static))
                    || t.equals(&Token::Keyword(Keyword::Field)) =>
            {
                let class_var_dec = handle_class_var_dec(parser);
                decs.push(class_var_dec);
            }
            Some(_) => {
                // not a class var dec
                break;
            }
            None => {
                panic!("EOF!")
            }
        }
    }

    return decs;
}

fn handle_subroutine_dec(parser: &mut TokenParser) -> ProgramElement {
    let mut subroutine_dec = NonTerminalElement::new(NonTerminalType::SubroutineDec);
    let keyword = ProgramElement::new(parser.consume_tok());
    subroutine_dec.add(keyword);
    let subroutine_type = match parser.peek_tok() {
        Some(Token::Keyword(Keyword::Void)) => ProgramElement::new(parser.consume_tok()),
        Some(_) => handle_declared_type(parser.consume_tok()),
        None => panic!("EOF!"),
    };
    subroutine_dec.add(subroutine_type);
    let subroutine_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
    subroutine_dec.add(subroutine_name);

    // (type varName *,type varName*);
    let l_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LParen));
    subroutine_dec.add(l_paren);
    let mut param_list = NonTerminalElement::new(NonTerminalType::ParamList);
    loop {
        match parser.consume_tok() {
            Some(Token::Symbol(Symbol::RParen)) => {
                // end param list
                subroutine_dec.add(ProgramElement::NonTerminal(param_list));
                subroutine_dec.add(ProgramElement::Terminal(Token::Symbol(Symbol::RParen)));
                break;
            }
            Some(Token::Symbol(Symbol::Comma)) => {
                // look for type varName
                param_list.add(ProgramElement::Terminal(Token::Symbol(Symbol::Comma)));
                continue;
            }
            Some(tok) => {
                // handle type varName
                let var_type = handle_declared_type(Some(tok));
                param_list.add(var_type);
                let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
                param_list.add(var_name);
            }
            None => panic!("EOF!"),
        }
    }

    // { varDecs*
    let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
    subroutine_dec.add(l_brace);
    loop {
        match parser.peek_tok() {
            Some(Token::Keyword(Keyword::Var)) => {
                let var_dec = handle_var_dec(parser);
                subroutine_dec.add(var_dec);
            }
            Some(_) => {
                // end of var decs
                break;
            }
            None => {
                panic!("EOF!")
            }
        }
    }
    // statements }
    let statements = handle_statements(parser);
    subroutine_dec.add(statements);
    let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
    subroutine_dec.add(r_brace);

    return ProgramElement::NonTerminal(subroutine_dec);
}

fn handle_var_dec(parser: &mut TokenParser) -> ProgramElement {
    let mut var_dec = NonTerminalElement::new(NonTerminalType::VarDec);

    // match var type varName
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Var));
    var_dec.add(keyword);
    let var_type = handle_declared_type(parser.consume_tok());
    var_dec.add(var_type);
    let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
    var_dec.add(var_name);

    // match ,varName*;
    loop {
        match parser.consume_tok() {
            Some(Token::Symbol(Symbol::Semicolon)) => {
                // end of var dec
                var_dec.add(ProgramElement::Terminal(Token::Symbol(Symbol::Semicolon)));
                break;
            }
            Some(Token::Symbol(Symbol::Comma)) => {
                var_dec.add(ProgramElement::Terminal(Token::Symbol(Symbol::Comma)));
                let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
                var_dec.add(var_name);
            }
            Some(_) => {
                panic!("Expected ; or another variable name!");
            }
            None => {
                panic!("EOF!");
            }
        }
    }

    return ProgramElement::NonTerminal(var_dec);
}

fn handle_subroutine_decs(parser: &mut TokenParser) -> Vec<ProgramElement> {
    let mut decs: Vec<ProgramElement> = Vec::new();
    loop {
        match parser.peek_tok() {
            Some(t)
                if t.equals(&Token::Keyword(Keyword::Constructor))
                    | t.equals(&Token::Keyword(Keyword::Method))
                    | t.equals(&Token::Keyword(Keyword::Function)) =>
            {
                let subroutine_dec = handle_subroutine_dec(parser);
                decs.push(subroutine_dec);
            }
            Some(_) => {
                // not a subroutine dec
                break;
            }
            None => {
                panic!("EOF!");
            }
        }
    }
    return decs;
}

// returns root of program/file
fn handle_class(parser: &mut TokenParser) -> NonTerminalElement {
    let mut class = NonTerminalElement::new(NonTerminalType::Class);

    let class_keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Class));
    class.add(class_keyword);
    let class_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
    class.add(class_name);
    let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
    class.add(r_brace);

    let mut class_var_decs = handle_class_var_decs(parser);
    class.add_vec(&mut class_var_decs);
    let mut subroutine_decs = handle_subroutine_decs(parser);
    class.add_vec(&mut subroutine_decs);

    let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
    class.add(l_brace);

    return class;
}

fn is_operation(tok: &Token) -> bool {
    match tok {
        Token::Symbol(Symbol::Add)
        | Token::Symbol(Symbol::Minus)
        | Token::Symbol(Symbol::Mult)
        | Token::Symbol(Symbol::Division)
        | Token::Symbol(Symbol::And)
        | Token::Symbol(Symbol::Or)
        | Token::Symbol(Symbol::LessThan)
        | Token::Symbol(Symbol::GreaterThan)
        | Token::Symbol(Symbol::Equal) => true,
        _ => false,
    }
}
fn handle_term(parser: &mut TokenParser) -> ProgramElement {
    let mut term = NonTerminalElement::new(NonTerminalType::Term);
    match parser.consume_tok() {
        Some(t)
            if t.equals(&Token::StringConst("".to_string()))
                || t.equals(&Token::IntConst("".to_string()))
                || t.equals(&Token::Keyword(Keyword::True))
                || t.equals(&Token::Keyword(Keyword::False))
                || t.equals(&Token::Keyword(Keyword::Null))
                || t.equals(&Token::Keyword(Keyword::This)) =>
        {
            // integerConstant | stringConstant | keywordConstant
            let constant = ProgramElement::Terminal(t);
            term.add(constant);
        }
        Some(t)
            if t.equals(&Token::Symbol(Symbol::Minus))
                || t.equals(&Token::Symbol(Symbol::Negation)) =>
        {
            // unaryOp term
            let unary = ProgramElement::Terminal(t);
            term.add(unary);
            let next_term = handle_term(parser);
            term.add(next_term);
        }
        Some(t) if t.equals(&Token::Identifier("".to_string())) => {
            // varName | varName[expr] | subroutineCall
            match parser.peek_tok() {
                Some(Token::Symbol(Symbol::LBrack)) => {
                    // varName[expr]
                    let var_name = ProgramElement::Terminal(t);
                    term.add(var_name);
                    let l_brack = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrack));
                    term.add(l_brack);
                    let expr = handle_expression(parser);
                    term.add(expr);
                    let r_brack = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrack));
                    term.add(r_brack);
                }
                Some(Token::Symbol(Symbol::LParen)) => {
                    // subroutineName (expressionList)
                    let subroutine_name = ProgramElement::Terminal(t);
                    term.add(subroutine_name);
                    let l_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LParen));
                    term.add(l_paren);
                    let expr_list = handle_expression_list(parser);
                    term.add(expr_list);
                    let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
                    term.add(r_paren);
                }
                Some(Token::Symbol(Symbol::Dot)) => {
                    // className|varName . subroutineName (expresionList)
                    let name = ProgramElement::Terminal(t);
                    term.add(name);
                    let dot = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Dot));
                    term.add(dot);
                    let subroutine_name =
                        match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
                    term.add(subroutine_name);
                    let l_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LParen));
                    term.add(l_paren);
                    let expr_list = handle_expression_list(parser);
                    term.add(expr_list);
                    let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
                    term.add(r_paren);
                }
                Some(_) => {
                    // varName
                    let var_name = ProgramElement::Terminal(t);
                    term.add(var_name);
                }
                None => {
                    panic!("EOF!");
                }
            }
        }
        Some(t) if t.equals(&Token::Symbol(Symbol::LParen)) => {
            // (expression)
            let l_paren = ProgramElement::Terminal(t);
            term.add(l_paren);
            let expr = handle_expression(parser);
            term.add(expr);
            let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
            term.add(r_paren);
        }
        Some(_) => {
            panic!("Received unexpected term in expression");
        }
        None => {
            panic!("EOF!");
        }
    };

    return ProgramElement::NonTerminal(term);
}

fn handle_expression_list(parser: &mut TokenParser) -> ProgramElement {
    let mut expr_list = NonTerminalElement::new(NonTerminalType::ExpressionList);
    // expression ,expression* ?
    loop {
        match parser.peek_tok() {
            Some(Token::Symbol(Symbol::RParen)) => {
                // end expr list
                break;
            }
            Some(Token::Symbol(Symbol::Comma)) => {
                // ,expression
                let comma = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Comma));
                expr_list.add(comma);
                let expr = handle_expression(parser);
                expr_list.add(expr);
            }
            Some(_) => {
                // expression
                let expr = handle_expression(parser);
                expr_list.add(expr);
            }
            None => {
                panic!("EOF!");
            }
        }
    }
    return ProgramElement::NonTerminal(expr_list);
}
fn handle_expression(parser: &mut TokenParser) -> ProgramElement {
    let mut expression = NonTerminalElement::new(NonTerminalType::Expression);
    // term opterm*
    let term = handle_term(parser);
    expression.add(term);

    loop {
        match parser.peek_tok() {
            Some(t) if is_operation(t) => {
                // op term
                let op = ProgramElement::new(parser.consume_tok());
                expression.add(op);
                let term = handle_term(parser);
                expression.add(term);
            }
            Some(_) => {
                // not op
                break;
            }
            None => {
                panic!("EOF!");
            }
        }
    }

    return ProgramElement::NonTerminal(expression);
}

fn handle_let_statement(parser: &mut TokenParser) -> ProgramElement {
    let mut let_statement = NonTerminalElement::new(NonTerminalType::LetStatement);

    // let varName
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Let));
    let_statement.add(keyword);
    let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
    let_statement.add(var_name);

    // [expr]?
    match parser.peek_tok() {
        Some(Token::Symbol(Symbol::LBrack)) => {
            let l_brack = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrack));
            let_statement.add(l_brack);
            let expr = handle_expression(parser);
            let_statement.add(expr);
            let r_brack = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrack));
            let_statement.add(r_brack);
        }
        Some(_) => {
            // no [expr]
        }
        None => {
            panic!("Reached EOF!");
        }
    }

    // = expr;
    let equal = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Equal));
    let_statement.add(equal);
    let expr = handle_expression(parser);
    let_statement.add(expr);
    let semicolon = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Semicolon));
    let_statement.add(semicolon);

    return ProgramElement::NonTerminal(let_statement);
}
fn handle_if_statement(parser: &mut TokenParser) -> ProgramElement {
    let mut if_statement = NonTerminalElement::new(NonTerminalType::IfStatement);

    // if(expr)
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::If));
    if_statement.add(keyword);
    let l_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LParen));
    if_statement.add(l_paren);
    let expr = handle_expression(parser);
    if_statement.add(expr);
    let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
    if_statement.add(r_paren);

    // {statements}
    let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
    if_statement.add(l_brace);
    let statements = handle_statements(parser);
    if_statement.add(statements);
    let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
    if_statement.add(r_brace);

    // else {statements} ?
    match parser.peek_tok() {
        Some(Token::Keyword(Keyword::Else)) => {
            let else_statement = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Else));
            if_statement.add(else_statement);
            let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
            if_statement.add(l_brace);
            let statements = handle_statements(parser);
            if_statement.add(statements);
            let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
            if_statement.add(r_brace);
        }
        Some(_) => {
            // no else statement
        }
        None => {
            panic!("EOF!")
        }
    }

    return ProgramElement::NonTerminal(if_statement);
}
fn handle_while_statement(parser: &mut TokenParser) -> ProgramElement {
    let mut while_statement = NonTerminalElement::new(NonTerminalType::WhileStatement);

    // while(expr)
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::While));
    while_statement.add(keyword);
    let l_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LParen));
    while_statement.add(l_paren);
    let expr = handle_expression(parser);
    while_statement.add(expr);
    let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
    while_statement.add(r_paren);

    // {statements}
    let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
    while_statement.add(l_brace);
    let statements = handle_statements(parser);
    while_statement.add(statements);
    let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
    while_statement.add(r_brace);

    return ProgramElement::NonTerminal(while_statement);
}
fn handle_do_statement(parser: &mut TokenParser) -> ProgramElement {
    // do subroutineCall
    let mut do_statement = NonTerminalElement::new(NonTerminalType::DoStatement);
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Do));
    do_statement.add(keyword);
    // let subroutine_call = handle_subroutine_call();
    // do_statement.add(subroutine_call);

    return ProgramElement::NonTerminal(do_statement);
}
fn handle_return_statement(parser: &mut TokenParser) -> ProgramElement {
    let mut return_statement = NonTerminalElement::new(NonTerminalType::ReturnStatement);
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Return));
    return_statement.add(keyword);
    match parser.consume_tok() {
        Some(Token::Symbol(Symbol::Semicolon)) => {
            // ;
            let semicolon = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Semicolon));
            return_statement.add(semicolon);
        }
        Some(_) => {
            // expr;
            let expr = handle_expression(parser);
            return_statement.add(expr);
            let semicolon = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Semicolon));
            return_statement.add(semicolon);
        }
        None => {}
    }

    return ProgramElement::NonTerminal(return_statement);
}

fn handle_statements(parser: &mut TokenParser) -> ProgramElement {
    let mut statements = NonTerminalElement::new(NonTerminalType::Statements);

    loop {
        match parser.peek_tok() {
            Some(Token::Keyword(Keyword::Let)) => {
                // let varName ?[expression]? = expression ;
                let let_statement = handle_let_statement(parser);
                statements.add(let_statement);
            }
            Some(Token::Keyword(Keyword::If)) => {
                // if (expression) {statements} else{statements}?
                let if_statement = handle_if_statement(parser);
                statements.add(if_statement);
            }
            Some(Token::Keyword(Keyword::While)) => {
                // while (expression) {statements}
                let while_statement = handle_while_statement(parser);
                statements.add(while_statement);
            }
            Some(Token::Keyword(Keyword::Return)) => {
                // return expression? ;
                let return_statement = handle_return_statement(parser);
                statements.add(return_statement);
            }
            Some(Token::Keyword(Keyword::Do)) => {
                // do subroutineCall ;
                let do_statement = handle_do_statement(parser);
                statements.add(do_statement);
            }
            Some(Token::Symbol(Symbol::RBrace)) => {
                // end of statements
                // don't consume rbrace
                break;
            }
            Some(_) => {
                panic!("Unexpected token for statement");
            }
            None => {
                panic!("End of tokens");
            }
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
