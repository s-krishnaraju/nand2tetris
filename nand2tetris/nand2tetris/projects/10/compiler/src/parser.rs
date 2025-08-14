use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use core::panic;
use std::collections::VecDeque;

pub struct TokenParser {
    pub tokens: VecDeque<Token>,
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
pub enum NonTerminalType {
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
pub enum ProgramElement {
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

    fn to_xml(&self) -> String {
        let mut xml = String::new();
        match self {
            Self::Terminal(tok) => {
                let s = tok.to_xml();
                xml.push_str(&s);
                xml.push_str("\n");
            }
            Self::NonTerminal(nt) => {
                let s = nt.to_xml();
                xml.push_str(&s);
            }
        }
        return xml;
    }
}

// non-terminal elements have children
// terminal just have a token
#[derive(Debug)]
pub struct NonTerminalElement {
    pub nt_type: NonTerminalType,
    pub children: Vec<ProgramElement>,
}

impl NonTerminalElement {
    pub fn new(nt_type: NonTerminalType) -> NonTerminalElement {
        return NonTerminalElement {
            nt_type: nt_type,
            children: Vec::new(),
        };
    }
    fn add(&mut self, elem: ProgramElement) {
        self.children.push(elem);
    }
    fn add_vec(&mut self, v: &mut Vec<ProgramElement>) {
        self.children.append(v);
    }

    fn _to_str(&self) -> &str {
        match &self.nt_type {
            NonTerminalType::Class => "class",
            NonTerminalType::ClassVarDec => "classVarDec",
            NonTerminalType::SubroutineDec => "subroutineDec",
            NonTerminalType::ParamList => "parameterList",
            NonTerminalType::SubroutineBody => "subroutineBody",
            NonTerminalType::VarDec => "varDec",
            NonTerminalType::Statements => "statements",
            NonTerminalType::IfStatement => "ifStatement",
            NonTerminalType::LetStatement => "letStatement",
            NonTerminalType::WhileStatement => "whileStatement",
            NonTerminalType::DoStatement => "doStatement",
            NonTerminalType::ReturnStatement => "returnStatement",
            NonTerminalType::ExpressionList => "expressionList",
            NonTerminalType::Expression => "expression",
            NonTerminalType::Term => "term",
        }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!("<{}>\n", self._to_str()));
        for elem in &self.children {
            let elem_xml = elem.to_xml();
            let mut indent = String::new();
            for line in elem_xml.lines() {
                indent.push_str("  ");
                indent.push_str(&line);
                indent.push_str("\n");
            }
            xml.push_str(&indent);
        }
        xml.push_str(&format!("</{}>\n", self._to_str()));
        return xml;
    }
}

// check if correct type of token
fn match_tok(parsed_tok: Option<Token>, tok: Token) -> ProgramElement {
    match parsed_tok {
        Some(t) if t == tok => ProgramElement::Terminal(t),
        Some(_) => {
            panic!("Parsed token doesn't match!");
        }
        None => {
            panic!("EOF!");
        }
    }
}

fn handle_class_var_dec(parser: &mut TokenParser) -> ProgramElement {
    let mut class_var_dec = NonTerminalElement::new(NonTerminalType::ClassVarDec);
    let keyword = ProgramElement::new(parser.consume_tok());
    class_var_dec.add(keyword);
    let var_type = handle_declared_type(parser.consume_tok());
    class_var_dec.add(var_type);
    let var_name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
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
        Some(Token::Keyword(k)) => match k {
            Keyword::Boolean | Keyword::Char | Keyword::Int => {
                ProgramElement::Terminal(Token::Keyword(k))
            }
            _ => {
                panic!("Expected boolean, char, or int")
            }
        },
        Some(Token::Identifier(s)) => {
            return ProgramElement::Terminal(Token::Identifier(s));
        }
        _ => panic!("Expected identifier or keyword"),
    }
}

fn handle_class_var_decs(parser: &mut TokenParser) -> Vec<ProgramElement> {
    let mut decs: Vec<ProgramElement> = Vec::new();
    loop {
        match parser.peek_tok() {
            Some(t)
                if t == &Token::Keyword(Keyword::Static)
                    || t == &Token::Keyword(Keyword::Field) =>
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
    let mut subroutine_body = NonTerminalElement::new(NonTerminalType::SubroutineBody);
    let l_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LBrace));
    subroutine_body.add(l_brace);
    loop {
        match parser.peek_tok() {
            Some(Token::Keyword(Keyword::Var)) => {
                let var_dec = handle_var_dec(parser);
                subroutine_body.add(var_dec);
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
    subroutine_body.add(statements);
    let r_brace = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RBrace));
    subroutine_body.add(r_brace);
    subroutine_dec.add(ProgramElement::NonTerminal(subroutine_body));

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
                if t == &Token::Keyword(Keyword::Constructor)
                    || t == &Token::Keyword(Keyword::Method)
                    || t == &Token::Keyword(Keyword::Function) =>
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
pub fn handle_class(parser: &mut TokenParser) -> NonTerminalElement {
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
fn handle_subroutine_call(parser: &mut TokenParser) -> Vec<ProgramElement> {
    let mut call: Vec<ProgramElement> = Vec::new();
    // subroutineName/className/varName handled outside of this func
    match parser.consume_tok() {
        // (expressionList)
        Some(Token::Symbol(Symbol::LParen)) => {
            call.push(ProgramElement::Terminal(Token::Symbol(Symbol::LParen)));
            let expr_list = handle_expression_list(parser);
            call.push(expr_list);
            let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
            call.push(r_paren);
        }
        Some(Token::Symbol(Symbol::Dot)) => {
            // . subroutineName (expresionList)
            call.push(ProgramElement::Terminal(Token::Symbol(Symbol::Dot)));
            let subroutine_name =
                match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
            call.push(subroutine_name);
            let l_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::LParen));
            call.push(l_paren);
            let expr_list = handle_expression_list(parser);
            call.push(expr_list);
            let r_paren = match_tok(parser.consume_tok(), Token::Symbol(Symbol::RParen));
            call.push(r_paren);
        }
        _ => {
            panic!("Expected first token ( or . in suboutine call");
        }
    };
    return call;
}
fn handle_term(parser: &mut TokenParser) -> ProgramElement {
    let mut term = NonTerminalElement::new(NonTerminalType::Term);
    match parser.consume_tok() {
        Some(t)
            if t == Token::StringConst("".to_string())
                || t == Token::IntConst("".to_string())
                || t == Token::Keyword(Keyword::True)
                || t == Token::Keyword(Keyword::False)
                || t == Token::Keyword(Keyword::Null)
                || t == Token::Keyword(Keyword::This) =>
        {
            // integerConstant | stringConstant | keywordConstant
            let constant = ProgramElement::Terminal(t);
            term.add(constant);
        }
        Some(t) if t == Token::Symbol(Symbol::Minus) || t == Token::Symbol(Symbol::Negation) => {
            // unaryOp term
            let unary = ProgramElement::Terminal(t);
            term.add(unary);
            let next_term = handle_term(parser);
            term.add(next_term);
        }
        Some(t) if t == Token::Identifier("".to_string()) => {
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

                Some(Token::Symbol(Symbol::LParen)) | Some(Token::Symbol(Symbol::Dot)) => {
                    // subroutineCall
                    let name = ProgramElement::Terminal(t);
                    term.add(name);
                    let mut subroutine_call = handle_subroutine_call(parser);
                    term.add_vec(&mut subroutine_call);
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
        Some(t) if t == Token::Symbol(Symbol::LParen) => {
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
    // do subroutineCall;
    let mut do_statement = NonTerminalElement::new(NonTerminalType::DoStatement);
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Do));
    do_statement.add(keyword);
    let name = match_tok(parser.consume_tok(), Token::Identifier("".to_string()));
    do_statement.add(name);
    let mut subroutine_call = handle_subroutine_call(parser);
    do_statement.add_vec(&mut subroutine_call);
    let semicolon = match_tok(parser.consume_tok(), Token::Symbol(Symbol::Semicolon));
    do_statement.add(semicolon);
    return ProgramElement::NonTerminal(do_statement);
}
fn handle_return_statement(parser: &mut TokenParser) -> ProgramElement {
    let mut return_statement = NonTerminalElement::new(NonTerminalType::ReturnStatement);
    let keyword = match_tok(parser.consume_tok(), Token::Keyword(Keyword::Return));
    return_statement.add(keyword);
    match parser.peek_tok() {
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
