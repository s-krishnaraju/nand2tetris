use crate::{
    lexer::{Keyword, Symbol, Token},
    parser::{NonTerminalElement, NonTerminalType, ProgramElement},
};
use core::panic;
use std::{collections::HashMap, env::var};

#[derive(Debug, Clone)]
enum Scope {
    Field,
    Static,
    Var,
    Arg,
}

#[derive(Clone, Debug)]
enum VarType {
    Int,
    Boolean,
    Char,
    Class(String),
}

#[derive(Debug)]
struct Variable {
    name: String,
    var_type: VarType,
    scope: Scope,
    index: u8,
}

#[derive(Debug)]
struct SymbolTable {
    table: HashMap<String, Variable>,
    field_offset: u8,
    static_offset: u8,
    var_offset: u8,
    arg_offset: u8,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
            field_offset: 0,
            static_offset: 0,
            var_offset: 0,
            arg_offset: 0,
        }
    }

    fn add(&mut self, name: &String, var_type: &VarType, scope: &Scope) {
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
            Variable {
                name: name.clone(),
                var_type: var_type.clone(),
                scope: scope.clone(),
                index: offset,
            },
        );
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
    let var_type = match t.children.get(1) {
        Some(ProgramElement::Terminal(t)) => match t {
            Token::Keyword(Keyword::Boolean) => VarType::Boolean,
            Token::Keyword(Keyword::Int) => VarType::Int,
            Token::Keyword(Keyword::Char) => VarType::Char,
            Token::Identifier(s) => VarType::Class(s.clone()),
            _ => panic!(),
        },
        _ => panic!(),
    };
    // handle varName ,varName*;
    let mut i = 2;
    loop {
        match t.children.get(i) {
            Some(ProgramElement::Terminal(t)) => match t {
                Token::Identifier(name) => {
                    class_table.add(name, &var_type, &scope);
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
    dbg!(class_table);
}

fn code_gen(tree: NonTerminalElement) {
    let mut class_table = SymbolTable::new();
    // handle class var decs
    for child in &tree.children {
        match child {
            ProgramElement::Terminal(Token::Identifier(class_name)) => {}
            ProgramElement::NonTerminal(t) => match t.nt_type {
                NonTerminalType::ClassVarDec => {
                    handle_class_var_dec(&mut class_table, t);
                }
                NonTerminalType::SubroutineDec => {
                    // generate code for each subroutine here
                    // pass in the class table so it can resolve field and static variables
                }
                _ => {}
            },
            _ => {}
        }
    }

    // handle subroutine decs
}
