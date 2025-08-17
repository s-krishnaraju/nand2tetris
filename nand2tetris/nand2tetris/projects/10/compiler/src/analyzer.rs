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
    if_labels: usize,
    while_labels: usize,
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
            if_labels: 0,
            while_labels: 0,
            name: String::new(),
        }
    }

    fn add(&mut self, name: &String, sym_type: &SymType, scope: &Scope) {
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
                sym_type: sym_type.clone(),
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

fn compile_class_var_dec(class_table: &mut SymbolTable, t: &NonTerminalElement) {
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

fn compile_subroutine_var_dec(table: &mut SymbolTable, body: &Vec<ProgramElement>) {
    let mut i = 0;
    let keyword = match body.get(i) {
        Some(ProgramElement::Terminal(Token::Keyword(Keyword::Var))) => Scope::Var,
        _ => {
            panic!();
        }
    };
    i += 1;
    let sym_type = match body.get(i) {
        Some(ProgramElement::Terminal(t)) => get_sym_type(t),
        _ => {
            panic!();
        }
    };
    i += 1;
    loop {
        match body.get(i) {
            Some(ProgramElement::Terminal(t)) => match t {
                Token::Symbol(Symbol::Semicolon) => {
                    break;
                }
                Token::Symbol(Symbol::Comma) => {}
                Token::Identifier(s) => {
                    table.add(s, &sym_type, &keyword);
                }
                _ => {
                    panic!()
                }
            },
            _ => {
                panic!()
            }
        }
        i += 1;
    }
}
fn compile_subroutine_dec(class_table: &mut SymbolTable, t: &Vec<ProgramElement>) -> String {
    let mut subroutine_table = SymbolTable::new();
    let mut i = 0;
    let mut bytecode = String::new();
    let subroutine_kind = t.get(i).unwrap();
    i += 1;
    let init_subroutine_bytecode = match subroutine_kind {
        ProgramElement::Terminal(Token::Keyword(k)) => match k {
            Keyword::Method => {
                // initialize the this pointer arg
                subroutine_table.add(
                    &String::from("this"),
                    &SymType::Class(class_table.name.clone()),
                    &Scope::Arg,
                );
                format!("push argument 0\npop pointer 0\n")
            }
            Keyword::Constructor => format!(
                "push constant {}\ncall Memory.alloc 1\npop pointer 0\n",
                class_table.field_offset
            ),
            Keyword::Function => format!(""),
            _ => panic!(),
        },
        _ => panic!(),
    };
    let subroutine_type = t.get(i).unwrap();
    i += 1;
    if let ProgramElement::Terminal(Token::Identifier(subroutine_name)) = t.get(i).unwrap() {
        subroutine_table.name = subroutine_name.to_string();
    }
    i += 1;
    // skip (
    i += 1;
    if let ProgramElement::NonTerminal(t) = t.get(i).unwrap() {
        compile_param_list(&mut subroutine_table, &t.children);
        i += 1;
    }
    // skip )
    i += 1;

    let subroutine_bytecode = match t.get(i) {
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::SubroutineBody => {
                compile_subroutine_body(class_table, &mut subroutine_table, &nt.children)
            }
            _ => panic!(),
        },
        _ => panic!(),
    };

    bytecode.push_str(&format!(
        "function {}.{} {}\n",
        class_table.name, &subroutine_table.name, &subroutine_table.var_offset
    ));
    bytecode.push_str(&init_subroutine_bytecode);
    bytecode.push_str(&subroutine_bytecode);
    return bytecode;
}

fn compile_expression_list(
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> (String, usize) {
    let mut cnt = 0;
    let mut bytecode = String::new();
    // (expr ,expr*)?
    for el in body {
        match el {
            ProgramElement::NonTerminal(nt) => match nt.nt_type {
                NonTerminalType::Expression => {
                    cnt += 1;
                    let expr = compile_expression(class_table, subroutine_table, &nt.children);
                    bytecode.push_str(&expr);
                }
                _ => panic!(),
            },
            ProgramElement::Terminal(Token::Symbol(Symbol::Comma)) => {}
            _ => panic!(),
        }
    }
    return (bytecode, cnt);
}
fn compile_term(
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
    term: &Vec<ProgramElement>,
) -> String {
    let mut i = 0;
    let mut bytecode = String::new();
    match term.get(i).unwrap() {
        ProgramElement::Terminal(t) => match t {
            Token::IntConst(i) => {
                // push constant i
                bytecode.push_str(&format!("push constant {}\n", i));
            }
            Token::StringConst(s) => {
                // call string constructor then do appendChar
                bytecode.push_str(&format!("push constant {}\n", s.len()));
                bytecode.push_str("call String.new 1\n");
                for ch in s.bytes() {
                    // TEST: THIS MIGHT BE WRONG!
                    bytecode.push_str(&format!("push constant {}\n", ch));
                    bytecode.push_str("call String.appendChar 2\n");
                }
            }
            Token::Keyword(Keyword::Null) | Token::Keyword(Keyword::False) => {
                bytecode.push_str("push constant 0\n");
            }
            Token::Keyword(Keyword::This) => {
                bytecode.push_str("push pointer 0\n");
            }
            Token::Keyword(Keyword::True) => {
                bytecode.push_str("push constant 1\n");
                bytecode.push_str("neg\n");
            }
            Token::Symbol(Symbol::LParen) => {
                // (expr)
                i += 1;
                let expr = match term.get(i) {
                    Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                        NonTerminalType::Expression => {
                            compile_expression(class_table, subroutine_table, &nt.children)
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                };

                bytecode.push_str(&expr);
            }
            Token::Symbol(Symbol::Minus) | Token::Symbol(Symbol::Not) => {
                // handle unaryOp term
                i += 1;
                // push new term to stack
                let new_term = match term.get(i) {
                    Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                        NonTerminalType::Term => {
                            compile_term(class_table, subroutine_table, &nt.children)
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                };

                bytecode.push_str(&new_term);

                // not or negative it
                if t == &Token::Symbol(Symbol::Minus) {
                    bytecode.push_str("neg\n");
                } else {
                    bytecode.push_str("not\n");
                }
            }
            Token::Identifier(name) => {
                // subroutineCall | varName[expr] | varName
                i += 1;
                match term.get(i) {
                    Some(ProgramElement::Terminal(Token::Symbol(s))) => {
                        match s {
                            Symbol::LBrack => {
                                // varName[expr]
                                i += 1;
                                let expr = match term.get(i) {
                                    Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                                        NonTerminalType::Expression => compile_expression(
                                            class_table,
                                            subroutine_table,
                                            &nt.children,
                                        ),
                                        _ => panic!(),
                                    },
                                    _ => panic!(),
                                };

                                bytecode.push_str(&expr);
                                // resolve varName from symbol table
                                bytecode.push_str(&format!(
                                    "push {}\n",
                                    resolve_symbol_variable(name, class_table, subroutine_table)
                                ));
                                bytecode.push_str("add\n");
                                bytecode.push_str("pop pointer 1\n");
                                bytecode.push_str("push that 0\n");
                            }
                            Symbol::LParen => {
                                // subroutineName(expressionList)
                                i += 1;
                                // we need to push this which is in arg 0 onto the stack
                                bytecode.push_str("push argument 0\n");
                                let (expr_list, num_args) = match term.get(i) {
                                    Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                                        NonTerminalType::ExpressionList => compile_expression_list(
                                            class_table,
                                            subroutine_table,
                                            &nt.children,
                                        ),
                                        _ => panic!(),
                                    },
                                    _ => panic!(),
                                };
                                bytecode.push_str(&expr_list);
                                // num_args + 1 b/c we push the this arg
                                bytecode.push_str(&format!(
                                    "call {}.{} {}\n",
                                    class_table.name,
                                    name,
                                    num_args + 1
                                ));
                            }
                            Symbol::Dot => {
                                // className/varName .subroutineName(expressionList)
                                i += 1;
                                let subroutine_name = match term.get(i) {
                                    Some(ProgramElement::Terminal(Token::Identifier(s))) => s,
                                    _ => panic!(),
                                };
                                i += 2;
                                let (expr_list, num_args) = match term.get(i) {
                                    Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                                        NonTerminalType::ExpressionList => compile_expression_list(
                                            class_table,
                                            subroutine_table,
                                            &nt.children,
                                        ),
                                        _ => panic!(),
                                    },
                                    _ => panic!(),
                                };

                                if subroutine_table.table.contains_key(name)
                                    || class_table.table.contains_key(name)
                                {
                                    // method so we push the obj addr as first arg
                                    bytecode.push_str(&format!(
                                        "push {}\n",
                                        resolve_symbol_variable(
                                            name,
                                            class_table,
                                            subroutine_table
                                        )
                                    ));
                                    // push rest of args
                                    bytecode.push_str(&expr_list);
                                    // call subroutine
                                    bytecode.push_str(&format!(
                                        "call {}.{} {}\n",
                                        resolve_class_name(name, class_table, subroutine_table),
                                        subroutine_name,
                                        num_args + 1 // +1 b/c of this arg
                                    ));
                                } else {
                                    // constructor/function
                                    // push args
                                    bytecode.push_str(&expr_list);
                                    // call subroutine
                                    bytecode.push_str(&format!(
                                        "call {}.{} {}\n",
                                        name, subroutine_name, num_args
                                    ));
                                }
                            }
                            _ => panic!(),
                        }
                    }
                    None => {
                        // varName
                        bytecode.push_str(&format!(
                            "push {}\n",
                            resolve_symbol_variable(name, class_table, subroutine_table)
                        ));
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
        _ => panic!(),
    };
    return bytecode;
}

fn compile_expression(
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut i = 0;
    let mut bytecode = String::new();
    loop {
        match body.get(i) {
            Some(ProgramElement::NonTerminal(nt)) => {
                // handle term
                let term = match nt.nt_type {
                    NonTerminalType::Term => {
                        compile_term(class_table, subroutine_table, &nt.children)
                    }
                    _ => panic!(),
                };
                bytecode.push_str(&term);
                i += 1;
            }
            Some(ProgramElement::Terminal(t)) => {
                // handle op term
                i += 1;
                // push term onto stack
                let term = match body.get(i) {
                    Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                        NonTerminalType::Term => {
                            compile_term(class_table, subroutine_table, &nt.children)
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                };
                bytecode.push_str(&term);
                // push op to stack
                let op = compile_op(t);
                bytecode.push_str(&op);
                i += 1;
            }
            None => break,
        }
    }

    return bytecode;
}

fn resolve_class_name(
    name: &String,
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
) -> String {
    if let Some(sym) = subroutine_table.table.get(name) {
        return match &sym.sym_type {
            SymType::Class(class_name) => class_name.to_string(),
            _ => panic!(),
        };
    } else if let Some(sym) = class_table.table.get(name) {
        return match &sym.sym_type {
            SymType::Class(class_name) => class_name.to_string(),
            _ => panic!(),
        };
    } else {
        panic!();
    }
}
fn resolve_symbol_variable(
    name: &String,
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
) -> String {
    if let Some(sym) = subroutine_table.table.get(name) {
        return match sym.scope {
            Scope::Var => format!("local {}", sym.index),
            Scope::Arg => format!("argument {}", sym.index),
            _ => panic!(),
        };
    } else if let Some(sym) = class_table.table.get(name) {
        return match sym.scope {
            Scope::Field => format!("this {}", sym.index),
            Scope::Static => format!("static {}", sym.index),
            _ => panic!(),
        };
    } else {
        panic!();
    }
}

fn compile_op(op: &Token) -> String {
    match op {
        Token::Symbol(Symbol::Add) => "add\n",
        Token::Symbol(Symbol::Minus) => "sub\n",
        Token::Symbol(Symbol::Mult) => "call Math.multiply 2\n",
        Token::Symbol(Symbol::Division) => "call Math.divide 2\n",
        Token::Symbol(Symbol::And) => "and\n",
        Token::Symbol(Symbol::Or) => "or\n",
        Token::Symbol(Symbol::LessThan) => "lt\n",
        Token::Symbol(Symbol::GreaterThan) => "gt\n",
        Token::Symbol(Symbol::Equal) => "eq\n",
        _ => panic!(),
    }
    .to_string()
}

fn compile_let_statement(
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    let mut i = 1;
    let var_name = match body.get(i) {
        Some(ProgramElement::Terminal(Token::Identifier(s))) => s,
        _ => panic!(),
    };
    i += 1;
    match body.get(i) {
        Some(ProgramElement::Terminal(Token::Symbol(Symbol::LBrack))) => {
            i += 1;
            let l_expr = match body.get(i) {
                Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                    NonTerminalType::Expression => {
                        compile_expression(class_table, subroutine_table, &nt.children)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            };
            bytecode.push_str(&l_expr);
            bytecode.push_str(&format!(
                "push {}\n",
                resolve_symbol_variable(var_name, class_table, subroutine_table)
            ));
            bytecode.push_str("add\n");

            i += 3;
            let r_expr = match body.get(i) {
                Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                    NonTerminalType::Expression => {
                        compile_expression(class_table, subroutine_table, &nt.children)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            };
            bytecode.push_str(&r_expr);
            bytecode.push_str("pop temp 0\n");
            bytecode.push_str("pop pointer 1\n");
            bytecode.push_str("push temp 0\n");
            bytecode.push_str("pop that 0\n");
        }
        Some(ProgramElement::Terminal(Token::Symbol(Symbol::Equal))) => {
            i += 1;
            let expr = match body.get(i) {
                Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                    NonTerminalType::Expression => {
                        compile_expression(class_table, subroutine_table, &nt.children)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            };
            bytecode.push_str(&expr);
            bytecode.push_str(&format!(
                "pop {}\n",
                resolve_symbol_variable(var_name, class_table, subroutine_table)
            ));
        }
        _ => panic!(),
    }

    return bytecode;
}
fn compile_return_statement(
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    match body.get(1) {
        Some(ProgramElement::Terminal(Token::Symbol(Symbol::Semicolon))) => {
            bytecode.push_str("push constant 0\n");
            bytecode.push_str("return\n");
        }
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::Expression => {
                let expr = compile_expression(class_table, subroutine_table, &nt.children);
                bytecode.push_str(&expr);
                bytecode.push_str("return\n");
            }
            _ => panic!(),
        },
        _ => panic!(),
    };
    return bytecode;
}

fn compile_do_statement(
    class_table: &SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    let mut i = 1;
    let name = match body.get(i) {
        Some(ProgramElement::Terminal(Token::Identifier(s))) => s,
        _ => panic!(),
    };
    i += 1;

    match body.get(i) {
        Some(ProgramElement::Terminal(Token::Symbol(Symbol::LParen))) => {
            // subroutineName(expressionList)
            i += 1;
            bytecode.push_str("push argument 0\n");
            let (expr_list, num_args) = match body.get(i) {
                Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                    NonTerminalType::ExpressionList => {
                        compile_expression_list(class_table, subroutine_table, &nt.children)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            };
            bytecode.push_str(&expr_list);
            // num_args + 1 b/c we push the this arg
            bytecode.push_str(&format!(
                "call {}.{} {}\n",
                class_table.name,
                name,
                num_args + 1
            ));
            // pop off the returned value
            bytecode.push_str("pop temp 0\n");
        }
        Some(ProgramElement::Terminal(Token::Symbol(Symbol::Dot))) => {
            // className/varName .subroutineName(expressionList)
            i += 1;
            let subroutine_name = match body.get(i) {
                Some(ProgramElement::Terminal(Token::Identifier(s))) => s,
                _ => panic!(),
            };
            i += 2;
            let (expr_list, num_args) = match body.get(i) {
                Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
                    NonTerminalType::ExpressionList => {
                        compile_expression_list(class_table, subroutine_table, &nt.children)
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            };

            if subroutine_table.table.contains_key(name) || class_table.table.contains_key(name) {
                // method so we push the obj addr as first arg
                bytecode.push_str(&format!(
                    "push {}\n",
                    resolve_symbol_variable(name, class_table, subroutine_table)
                ));
                // push rest of args
                bytecode.push_str(&expr_list);
                // call subroutine
                bytecode.push_str(&format!(
                    "call {}.{} {}\n",
                    resolve_class_name(name, class_table, subroutine_table),
                    subroutine_name,
                    num_args + 1 // +1 b/c of this arg
                ));
                // pop off the returned value
                bytecode.push_str("pop temp 0\n");
            } else {
                // constructor/function
                // push args
                bytecode.push_str(&expr_list);
                // call subroutine
                bytecode.push_str(&format!("call {}.{} {}\n", name, subroutine_name, num_args));
                // pop off the returned value
                bytecode.push_str("pop temp 0\n");
            }
        }
        _ => panic!(),
    }

    return bytecode;
}

fn compile_while_statement(
    class_table: &mut SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    let mut i = 2;
    let expr = match body.get(i) {
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::Expression => {
                compile_expression(class_table, subroutine_table, &nt.children)
            }
            _ => panic!(),
        },
        _ => panic!(),
    };
    i += 3;
    let statements = match body.get(i) {
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::Statements => {
                compile_statements(class_table, subroutine_table, &nt.children)
            }
            _ => panic!(),
        },
        _ => panic!(),
    };

    class_table.while_labels += 1;
    bytecode.push_str(&format!("label WHILE_START{}\n", class_table.while_labels));
    bytecode.push_str(&expr);
    bytecode.push_str("neg\n");
    bytecode.push_str(&format!("if-goto WHILE_END{}\n", class_table.while_labels));
    bytecode.push_str(&statements);
    bytecode.push_str(&format!("goto WHILE_START{}\n", class_table.while_labels));
    bytecode.push_str(&format!("label WHILE_END{}\n", class_table.while_labels));
    return bytecode;
}
fn compile_if_statement(
    class_table: &mut SymbolTable,
    subroutine_table: &SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    let mut i = 2;
    let expr = match body.get(i) {
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::Expression => {
                compile_expression(class_table, subroutine_table, &nt.children)
            }
            _ => panic!(),
        },
        _ => panic!(),
    };
    i += 3;
    let statements = match body.get(i) {
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::Statements => {
                compile_statements(class_table, subroutine_table, &nt.children)
            }
            _ => panic!(),
        },
        _ => panic!(),
    };
    i += 4;

    // handle else statements if they exist
    let else_statements = match body.get(i) {
        Some(ProgramElement::NonTerminal(nt)) => match nt.nt_type {
            NonTerminalType::Statements => Some(compile_statements(
                class_table,
                subroutine_table,
                &nt.children,
            )),
            _ => panic!(),
        },
        None => None,
        _ => panic!(),
    };

    // gen VM code
    // handle branching
    // need to generate unique labels
    class_table.if_labels += 1;
    bytecode.push_str(&expr);
    bytecode.push_str("neg\n");
    bytecode.push_str(&format!("if-goto IF_END{}\n", class_table.if_labels));
    bytecode.push_str(&statements);
    bytecode.push_str(&format!("goto ELSE_END{}\n", class_table.if_labels));
    bytecode.push_str(&format!("label IF_END{}\n", class_table.if_labels));
    if let Some(s) = else_statements {
        bytecode.push_str(&s);
    }
    bytecode.push_str(&format!("label ELSE_END{}\n", class_table.if_labels));

    return bytecode;
}

fn compile_statements(
    class_table: &mut SymbolTable,
    subroutine_table: &SymbolTable,
    statements: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    for s in statements {
        let statement_bytecode = match s {
            ProgramElement::NonTerminal(nt) => match nt.nt_type {
                NonTerminalType::IfStatement => {
                    compile_if_statement(class_table, subroutine_table, &nt.children)
                }
                NonTerminalType::ReturnStatement => {
                    compile_return_statement(class_table, subroutine_table, &nt.children)
                }
                NonTerminalType::DoStatement => {
                    compile_do_statement(class_table, subroutine_table, &nt.children)
                }
                NonTerminalType::LetStatement => {
                    compile_let_statement(class_table, subroutine_table, &nt.children)
                }
                NonTerminalType::WhileStatement => {
                    compile_while_statement(class_table, subroutine_table, &nt.children)
                }
                _ => panic!(),
            },
            _ => panic!(),
        };
        bytecode.push_str(&statement_bytecode);
    }
    return bytecode;
}
fn compile_subroutine_body(
    class_table: &mut SymbolTable,
    subroutine_table: &mut SymbolTable,
    body: &Vec<ProgramElement>,
) -> String {
    let mut bytecode = String::new();
    for b in body {
        match b {
            ProgramElement::NonTerminal(nt) => match nt.nt_type {
                NonTerminalType::VarDec => {
                    compile_subroutine_var_dec(subroutine_table, &nt.children);
                }
                NonTerminalType::Statements => {
                    let statements_bytecode =
                        compile_statements(class_table, subroutine_table, &nt.children);
                    bytecode.push_str(&statements_bytecode);
                }
                _ => panic!(),
            },
            ProgramElement::Terminal(t) => match t {
                Token::Symbol(Symbol::RBrace) | Token::Symbol(Symbol::LBrace) => {}
                _ => panic!(),
            },
        }
    }
    return bytecode;
}

fn compile_param_list(table: &mut SymbolTable, params: &Vec<ProgramElement>) {
    let mut i = 0;
    loop {
        let p = params.get(i);
        match p {
            Some(ProgramElement::Terminal(Token::Symbol(Symbol::Comma))) => {
                i += 1;
                let sym_type = match params.get(i) {
                    Some(ProgramElement::Terminal(t)) => get_sym_type(t),
                    _ => panic!(),
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
            _ => panic!(),
        }
    }
}

pub fn analyze(tree: NonTerminalElement) {
    let mut class_table = SymbolTable::new();
    let mut bytecode = String::new();
    // compile class var decs
    for child in &tree.children {
        match child {
            ProgramElement::Terminal(Token::Identifier(class_name)) => {
                class_table.name = class_name.to_string();
            }
            ProgramElement::NonTerminal(t) => match t.nt_type {
                NonTerminalType::ClassVarDec => {
                    compile_class_var_dec(&mut class_table, t);
                }
                NonTerminalType::SubroutineDec => {
                    let subroutine_bytecode = compile_subroutine_dec(&mut class_table, &t.children);
                    bytecode.push_str(&subroutine_bytecode);
                }
                _ => {}
            },
            _ => {}
        }
    }
    print!("{}", bytecode);
}
