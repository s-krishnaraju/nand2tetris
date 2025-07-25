use std::collections::VecDeque;
use std::env;
use std::fs;

const BASE_STACK_ADDR: u16 = 256;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Can't read file!");

    let asm = translate_bytecode(&contents);
    let file_path = file_path.replace(".vm", ".asm");
    fs::write(file_path, &asm);
    println!("{}", &asm);
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
    const TMP: &str = "tmp";
    const STATIC: &str = "static";
    const POINTER: &str = "pointer";
    const THIS: &str = "this";
    const THAT: &str = "that";
}

fn translate_bytecode(contents: &str) -> String {
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
                push(segment, idx)
            }
            OP::POP => {
                let segment = tokens.pop_front().unwrap();
                let idx = tokens.pop_front().unwrap();
                pop(segment, idx)
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

        asm.push_str(&instructions);
    }

    return asm;
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

fn make_comparison(count: usize) -> String {
    format!("\nD=0\n@ENDTRUE{count}\n0;JMP\n(TRUE{count})\nD=-1\n(ENDTRUE{count})")
}

fn greater_than(count: usize) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());

    let pop_and_or = format!("@SP\nM=M-1\n@SP\nA=M\nD=D|M\n");

    asm.push_str(&pop_and_or);
    asm.push_str(&push_to_stack());
    return asm;
}

fn less_than(count: usize) -> String {
    panic!()
}

fn equal_to(count: usize) -> String {
    let mut asm = String::new();
    asm
}

fn or() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_or = format!("@SP\nM=M-1\n@SP\nA=M\nD=D|M\n");
    asm.push_str(&pop_and_or);
    asm.push_str(&push_to_stack());
    return asm;
}

fn and() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_and = format!("@SP\nM=M-1\n@SP\nA=M\nD=D&M\n");
    asm.push_str(&pop_and_and);
    asm.push_str(&push_to_stack());
    return asm;
}

fn sub() -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_sub = format!("@SP\nM=M-1\n@SP\nA=M\nD=D-M\n");
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

fn pop(segment: &str, idx: &str) -> String {
    if segment == SEGMENT::CONSTANT {
        panic!("Received constant as segment for pop");
    }

    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    asm.push_str(&load_segment(segment, idx));
    let write_to_segment = format!("M=D\n");
    asm.push_str(&write_to_segment);
    return asm;
}

fn push(segment: &str, idx: &str) -> String {
    let mut asm = String::new();

    let load_to_d = if segment == SEGMENT::CONSTANT {
        format!("@{idx}\nD=A\n")
    } else {
        format!("{}\nD=M\n", load_segment(segment, idx))
    };

    asm.push_str(&load_to_d);

    // overwrite current SP w/ D reg and then increment
    let push_to_stack = push_to_stack();

    asm.push_str(&push_to_stack);
    return asm;
}

// Loads into A register addr of segment
fn load_segment(segment: &str, idx: &str) -> String {
    let offset = idx.parse::<u8>().expect("Failed to parse segment value");
    let add_offset = format!("D=A\nA={offset}\nD=D+A\nA=D\n");

    return match segment {
        SEGMENT::LCL => {
            format!("@LCL\n{add_offset}")
        }
        SEGMENT::ARG => {
            format!("@ARG\n{add_offset}")
        }
        SEGMENT::THIS => {
            format!("@THIS\n{add_offset}")
        }
        SEGMENT::THAT => {
            format!("@THAT\n{add_offset}")
        }
        SEGMENT::TMP => {
            let addr = 5 + offset;
            format!("@{addr}\n")
        }
        SEGMENT::STATIC => {
            // use fname instead of static
            format!("@static.{idx}\n")
        }
        SEGMENT::POINTER => match offset {
            0 => "@THIS\n".to_string(),
            1 => "@THAT\n".to_string(),
            _ => panic!("Couldn't match offset for pointer"),
        },
        _ => panic!("Couldn't match segment"),
    };
}

// Decrement then pop val to D reg
fn pop_from_stack() -> String {
    format!("@SP\nM=M-1\n@SP\nA=M\nD=M\n")
}

// Write from D reg to stack and increment
fn push_to_stack() -> String {
    format!("@SP\nA=M\nM=D\n@SP\nM=M+1\n")
}
