use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_name = file_path.split("/").last().unwrap().replace(".vm", "");

    let contents = fs::read_to_string(file_path).expect("Can't read file!");
    let asm = translate_bytecode(&contents, &file_name);

    let file_path = file_path.replace(".vm", ".asm");
    fs::write(file_path, &asm);
}

struct OP {}
impl OP {
    const PUSH: &str = "push";
    const POP: &str = "pop";
    const ADD: &str = "add";
    const SUB: &str = "sub";
    const NEG: &str = "neg";
    const GT: &str = "gt";
    const LT: &str = "lt";
    const EQ: &str = "eq";
    const AND: &str = "and";
    const OR: &str = "or";
    const NOT: &str = "not";
    const GOTO: &str = "goto";
    const IF: &str = "if";
    const FUNCTION: &str = "function";
    const RETURN: &str = "return";
    const CALL: &str = "call";
}

struct SEGMENT {}
impl SEGMENT {
    const CONSTANT: &str = "constant";
    const LCL: &str = "local";
    const ARG: &str = "argument";
    const TEMP: &str = "temp";
    const STATIC: &str = "static";
    const POINTER: &str = "pointer";
    const THIS: &str = "this";
    const THAT: &str = "that";

    fn translate(segment: &str) -> String {
        match segment {
            SEGMENT::LCL => "LCL",
            SEGMENT::ARG => "ARG",
            SEGMENT::THIS => "THIS",
            SEGMENT::THAT => "THAT",
            _ => "",
        }
        .to_string()
    }
}

fn translate_bytecode(contents: &str, file_name: &str) -> String {
    let mut asm = String::new();
    let mut comparison_count = 0;

    for line in contents.lines() {
        let mut tokens: VecDeque<&str> = line.trim().split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        let first = tokens.pop_front().unwrap();
        if first.starts_with("/") {
            continue;
        }

        match first {
            OP::GT | OP::EQ | OP::LT => {
                comparison_count += 1;
            }
            _ => {}
        }

        let instructions = match first {
            OP::PUSH => {
                let segment = tokens.pop_front().unwrap();
                let idx = tokens.pop_front().unwrap();
                push(segment, idx, file_name)
            }
            OP::POP => {
                let segment = tokens.pop_front().unwrap();
                let idx = tokens.pop_front().unwrap();
                pop(segment, idx, file_name)
            }
            OP::GT => greater_than(comparison_count),
            OP::LT => less_than(comparison_count),
            OP::EQ => equal_to(comparison_count),
            OP::ADD => add(),
            OP::SUB => sub(),
            OP::NEG => neg(),
            OP::AND => and(),
            OP::OR => or(),
            OP::NOT => not(),
            // OP::GOTO => {}
            // OP::IF => {}
            // OP::FUNCTION => {}
            // OP::RETURN => {}
            // OP::CALL => {}
            _ => {
                panic!("Unable to match line: {}", line);
            }
        };
        asm.push_str(&format!("//{}\n", &line));
        asm.push_str(&instructions);
    }

    asm.push_str(&add_terminator());
    return asm;
}

fn call() {}
fn return_op() {}
fn if_op() {}
fn goto() {}
fn function() {}

fn add_terminator() -> String {
    format!("(END)\n@END\n0;JMP\n")
}

fn neg() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let neg = format!("D=-D\n");
    asm.push_str(&neg);
    asm.push_str(&push_to_stack());
    return asm;
}

fn not() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let not = format!("D=!D\n");
    asm.push_str(&not);
    asm.push_str(&push_to_stack());
    return asm;
}

// Sets D=-1 if true else D=0
fn set_compare_result(count: usize) -> String {
    format!("D=0\n@FALSE{count}\n0;JMP\n(TRUE{count})\nD=-1\n(FALSE{count})\n")
}

fn greater_than(count: usize) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_jmp = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n@TRUE{count}\nD;JGT\n");
    asm.push_str(&pop_and_jmp);
    let comparison = set_compare_result(count);
    asm.push_str(&comparison);
    asm.push_str(&push_to_stack());
    return asm;
}

fn less_than(count: usize) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_jmp = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n@TRUE{count}\nD;JLT\n");
    asm.push_str(&pop_and_jmp);
    let comparison = set_compare_result(count);
    asm.push_str(&comparison);
    asm.push_str(&push_to_stack());
    return asm;
}

fn equal_to(count: usize) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_jmp = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n@TRUE{count}\nD;JEQ\n");
    asm.push_str(&pop_and_jmp);
    let comparison = set_compare_result(count);
    asm.push_str(&comparison);
    asm.push_str(&push_to_stack());
    return asm;
}

fn or() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_or = format!("@SP\nM=M-1\n@SP\nA=M\nD=M|D\n");
    asm.push_str(&pop_and_or);
    asm.push_str(&push_to_stack());
    return asm;
}

fn and() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_and = format!("@SP\nM=M-1\n@SP\nA=M\nD=M&D\n");
    asm.push_str(&pop_and_and);
    asm.push_str(&push_to_stack());
    return asm;
}

fn sub() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_sub = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n");
    asm.push_str(&pop_and_sub);
    asm.push_str(&push_to_stack());
    return asm;
}

fn add() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_add = format!("@SP\nM=M-1\n@SP\nA=M\nD=D+M\n");
    asm.push_str(&pop_and_add);
    asm.push_str(&push_to_stack());
    return asm;
}

fn pop(segment: &str, idx: &str, file_name: &str) -> String {
    if segment == SEGMENT::CONSTANT {
        panic!("Can't pop to a constant!");
    }

    let mut asm = String::new();
    let offset = idx.parse::<u8>().expect("Failed to parse segment idx");

    match segment {
        SEGMENT::LCL | SEGMENT::ARG | SEGMENT::THIS | SEGMENT::THAT => {
            let segment = SEGMENT::translate(segment);
            let add_offset = format!("@{offset}\nD=A\n@{segment}\nM=M+D\n");
            asm.push_str(&add_offset);
            asm.push_str(&pop_from_stack());
            let write_to_segment = &format!("@{segment}\nA=M\nM=D\n");
            asm.push_str(&write_to_segment);
            let sub_offset = format!("@{offset}\nD=A\n@{segment}\nM=M-D\n");
            asm.push_str(&sub_offset);
        }
        SEGMENT::TEMP => {
            asm.push_str(&pop_from_stack());
            let addr = 5 + offset;
            asm.push_str(&format!("@{addr}\nM=D\n"));
        }
        SEGMENT::STATIC => {
            asm.push_str(&pop_from_stack());
            asm.push_str(&format!("@{file_name}.{idx}\nM=D\n"));
        }
        SEGMENT::POINTER => {
            asm.push_str(&pop_from_stack());
            let segment = match offset {
                0 => "THIS\n".to_string(),
                1 => "THAT\n".to_string(),
                _ => panic!("Couldn't match offset for pointer"),
            };
            asm.push_str(&format!("@{segment}\nM=D\n"));
        }
        _ => panic!("Invalid arg for pop instruction"),
    };

    return asm;
}
fn push(segment: &str, idx: &str, file_name: &str) -> String {
    let mut asm = String::new();

    if segment == SEGMENT::CONSTANT {
        let load_from_constant = format!("@{idx}\nD=A\n");
        asm.push_str(&load_from_constant);
    } else {
        let offset = idx.parse::<u8>().expect("Failed to parse segment value");
        let load_from_segment = match segment {
            SEGMENT::LCL | SEGMENT::ARG | SEGMENT::THIS | SEGMENT::THAT => {
                let segment = SEGMENT::translate(segment);
                let offset_addr = format!("@{offset}\nD=A\n@{segment}\nD=D+M\nA=D\n");
                offset_addr
            }
            SEGMENT::TEMP => {
                let addr = 5 + offset;
                format!("@{addr}\n")
            }
            SEGMENT::STATIC => {
                format!("@{file_name}.{idx}\n")
            }
            SEGMENT::POINTER => match offset {
                0 => "@THIS\n".to_string(),
                1 => "@THAT\n".to_string(),
                _ => panic!("Couldn't match offset for pointer"),
            },
            _ => "".to_string(),
        };
        asm.push_str(&load_from_segment);
        asm.push_str("D=M\n");
    }

    let push_to_stack = push_to_stack();
    asm.push_str(&push_to_stack);
    return asm;
}

// Decrement then pop val to D reg
fn pop_from_stack() -> String {
    format!("@SP\nM=M-1\n@SP\nA=M\nD=M\n")
}

// Write from D reg to stack and increment
fn push_to_stack() -> String {
    format!("@SP\nA=M\nM=D\n@SP\nM=M+1\n")
}
