use crate::{
    lexer::{Keyword, Symbol, Token},
    parser::{NonTerminalElement, NonTerminalType, ProgramElement},
};
use core::panic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Scope {
    Field,
    Static,
    Var,
    Arg,
}

#[derive(Clone, Debug)]
enum SymType {
    Int,
    Boolean,
    Char,
    Class(String),
}

#[derive(Debug)]
struct SymbolicVariable {
    name: String,
    sym_type: SymType,
    scope: Scope,
    index: u8,
}

#[derive(Debug)]
struct SymbolTable {
    table: HashMap<String, SymbolicVariable>,
    field_offset: u8,
    static_offset: u8,
    var_offset: u8,
    arg_offset: u8,
    name: String,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
            field_offset: 0,
            static_offset: 0,
            var_offset: 0,
            arg_offset: 0,
            name: String::new(),
        }
    }

    fn add(&mut self, name: &String, var_type: &SymType, scope: &Scope) {
        let offset = match scope {
            Scope::Field => {
                self.field_offset += 1;
                self.field_offset - 1
            }
            Scope::Static => {
                self.static_offset += 1;
                self.static_offset - 1
            }
            Scope::Var => {
                self.var_offset += 1;
                self.var_offset - 1
            }
            Scope::Arg => {
                self.arg_offset += 1;
                self.arg_offset - 1
            }
        };

        self.table.insert(
            name.clone(),
            SymbolicVariable {
                name: name.clone(),
                sym_type: var_type.clone(),
                scope: scope.clone(),
                index: offset,
            },
        );
    }
}

fn get_sym_type(t: &Token) -> SymType {
    match t {
        Token::Keyword(Keyword::Boolean) => SymType::Boolean,
        Token::Keyword(Keyword::Int) => SymType::Int,
        Token::Keyword(Keyword::Char) => SymType::Char,
        Token::Identifier(s) => SymType::Class(s.clone()),
        _ => panic!(),
    }
}

fn handle_class_var_dec(class_table: &mut SymbolTable, t: &NonTerminalElement) {
    // populate class symbol table
    let scope = match t.children.first() {
        Some(ProgramElement::Terminal(Token::Keyword(Keyword::Field))) => Scope::Field,
        Some(ProgramElement::Terminal(Token::Keyword(Keyword::Static))) => Scope::Static,
        _ => {
            panic!()
        }
    };
    let sym_type = match t.children.get(1) {
        Some(ProgramElement::Terminal(t)) => get_sym_type(t),
        _ => panic!(),
    };
    // handle varName ,varName*;
    let mut i = 2;
    loop {
        match t.children.get(i) {
            Some(ProgramElement::Terminal(t)) => match t {
                Token::Identifier(name) => {
                    class_table.add(name, &sym_type, &scope);
                }
                Token::Symbol(Symbol::Semicolon) => {
                    break;
                }
                Token::Symbol(Symbol::Comma) => {}
                _ => panic!(),
            },
            _ => {
                panic!();
            }
        };
        i += 1;
    }
}

fn handle_subroutine_dec(class_table: &SymbolTable, t: &Vec<ProgramElement>) {
    let mut subroutine_table = SymbolTable::new();
    let mut i = 0;
    let subroutine_kind = t.get(i).unwrap();
    i += 1;
    match subroutine_kind {
        ProgramElement::Terminal(Token::Keyword(k)) => match k {
            Keyword::Method => {
                // This pointer to field vars
                subroutine_table.add(
                    &String::from("this"),
                    &SymType::Class(class_table.name.clone()),
                    &Scope::Arg,
                );
            }
            Keyword::Constructor => {}
            Keyword::Function => {}
            _ => {
                panic!();
            }
        },
        _ => {
            panic!();
        }
    }
    let subroutine_type = t.get(i).unwrap();
    i += 1;
    if let ProgramElement::Terminal(Token::Identifier(subroutine_name)) = t.get(i).unwrap() {
        subroutine_table.name = subroutine_name.to_string();
    }
    i += 1;
    // skip (
    i += 1;
    if let ProgramElement::NonTerminal(t) = t.get(i).unwrap() {
        handle_param_list(&mut subroutine_table, &t.children);
        i += 1;
    }
    // skip )
    i += 1;

    if let ProgramElement::NonTerminal(t) = t.get(i).unwrap() {
        handle_subroutine_body();
    }

    dbg!(subroutine_table);
}

fn handle_subroutine_body(table: &mut SymbolTable, body: &Vec<ProgramElement>) {
    for b in body {
        match b {
            ProgramElement::NonTerminal(t) => match t.nt_type {
                NonTerminalType::VarDec => {
                    // handle var decs add them to symbol table
                }
                NonTerminalType::IfStatement => {}
                NonTerminalType::LetStatement => {}
                NonTerminalType::DoStatement => {}
                NonTerminalType::ReturnStatement => {}
                NonTerminalType::WhileStatement => {}
                _ => {
                    panic!();
                }
            },
            _ => {}
        }
    }
}

fn handle_param_list(table: &mut SymbolTable, params: &Vec<ProgramElement>) {
    let mut i = 0;
    loop {
        let p = params.get(i);
        match p {
            Some(ProgramElement::Terminal(Token::Symbol(Symbol::Comma))) => {
                i += 1;
                let sym_type = match params.get(i) {
                    Some(ProgramElement::Terminal(t)) => get_sym_type(t),
                    _ => {
                        panic!()
                    }
                };
                i += 1;
                if let ProgramElement::Terminal(Token::Identifier(var_name)) =
                    params.get(i).unwrap()
                {
                    table.add(var_name, &sym_type, &Scope::Arg);
                    i += 1;
                } else {
                    panic!();
                }
            }
            Some(ProgramElement::Terminal(t)) => {
                let sym_type = get_sym_type(t);
                i += 1;
                if let ProgramElement::Terminal(Token::Identifier(var_name)) =
                    params.get(i).unwrap()
                {
                    table.add(var_name, &sym_type, &Scope::Arg);
                    i += 1;
                } else {
                    panic!();
                }
            }
            None => {
                break;
            }
            _ => {
                panic!();
            }
        }
    }
}

pub fn code_gen(tree: NonTerminalElement) {
    let mut class_table = SymbolTable::new();
    // handle class var decs
    for child in &tree.children {
        match child {
            ProgramElement::Terminal(Token::Identifier(class_name)) => {
                class_table.name = class_name.to_string();
            }
            ProgramElement::NonTerminal(t) => match t.nt_type {
                NonTerminalType::ClassVarDec => {
                    handle_class_var_dec(&mut class_table, t);
                }
                NonTerminalType::SubroutineDec => {
                    handle_subroutine_dec(&class_table, &t.children);
                }
                _ => {}
            },
            _ => {}
        }
    }

    // handle subroutine decs
}
