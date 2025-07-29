use std::collections::VecDeque;
use std::env;
use std::fs;

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
    const LABEL: &str = "label";
    const GOTO: &str = "goto";
    const IF_GOTO: &str = "if-goto";
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut call_counter: u32 = 0;
    let mut comparison_counter: u32 = 0;

    let mut asm = String::new();

    if file_path.contains(".vm") {
        let contents = fs::read_to_string(file_path).expect("Can't read file!");
        let file_name = file_path.split("/").last().unwrap().replace(".vm", "");
        let translated_bytecode = translate_bytecode(
            &contents,
            &file_name,
            &mut comparison_counter,
            &mut call_counter,
        );
        asm.push_str(&translated_bytecode);
        asm.push_str(&add_terminator());
        let file = file_path.replace(".vm", ".asm");
        fs::write(file, &asm).expect("Couldn't write to file");
    } else {
        let paths = fs::read_dir(file_path).expect("Couldn't read directory");
        asm.push_str(&call_sys_init());
        for path in paths {
            if let Ok(path) = path {
                if let Ok(file_name) = path.file_name().into_string() {
                    if file_name.contains(".vm") {
                        dbg!(&file_name);
                        let contents = fs::read_to_string(path.path()).expect("Can't read file!");
                        let file_name = file_name.replace(".vm", "");
                        let translated_bytecode = translate_bytecode(
                            &contents,
                            &file_name,
                            &mut comparison_counter,
                            &mut call_counter,
                        );
                        asm.push_str(&translated_bytecode);
                    }
                }
            }
        }

        asm.push_str(&add_terminator());
        let mut split: Vec<&str> = file_path.trim().split("/").collect();
        dbg!(&split);

        if let Some(dir_name) = split.last() {
            if dir_name.is_empty() {
                split.pop();
            }
            let file_name = split.last().expect("Couldn't get directory name");
            let mut file_path = split.join("/");
            file_path.push_str(&format!("/{file_name}.asm"));
            fs::write(file_path, &asm).expect("Couldn't write to file");
        }
    }
}

const BASE_STACK_ADDR: usize = 256;
fn call_sys_init() -> String {
    let mut asm = String::new();
    let init_sp = format!("@{BASE_STACK_ADDR}\nD=A\n@SP\nM=D\n");
    let count: u32 = 0;
    asm.push_str(&init_sp);
    asm.push_str(&write_call(&count, "INIT", "Sys.init", 0));
    asm
}

fn translate_bytecode(
    contents: &str,
    file_name: &str,
    comparison_counter: &mut u32,
    call_counter: &mut u32,
) -> String {
    let mut asm = String::new();
    let mut func_name = String::new();

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
                *comparison_counter += 1;
            }
            OP::CALL => {
                *call_counter += 1;
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
            OP::GT => greater_than(file_name, *comparison_counter),
            OP::LT => less_than(file_name, *comparison_counter),
            OP::EQ => equal_to(file_name, *comparison_counter),
            OP::ADD => add(),
            OP::SUB => sub(),
            OP::NEG => neg(),
            OP::AND => and(),
            OP::OR => or(),
            OP::NOT => not(),
            OP::LABEL => {
                let label = tokens.pop_front().expect("Expected label");
                write_label(label, &func_name)
            }
            OP::GOTO => {
                let label = tokens.pop_front().expect("Expected label");
                write_goto(label, &func_name)
            }
            OP::IF_GOTO => {
                let label = tokens.pop_front().expect("Expected label");
                write_if_goto(label, &func_name)
            }
            OP::FUNCTION => {
                func_name = tokens
                    .pop_front()
                    .expect("No function name provided")
                    .to_string();
                let num_vars = tokens
                    .pop_front()
                    .expect("Num of local variables not provided")
                    .parse::<usize>()
                    .expect("Provide a number for the # of local vars");

                write_function(&func_name, num_vars)
            }
            OP::RETURN => write_return(),
            OP::CALL => {
                let called_func = tokens.pop_front().expect("No function name provided");
                let num_args = tokens
                    .pop_front()
                    .expect("# of args not provided")
                    .parse::<usize>()
                    .expect("Provide a number for # of args");
                write_call(&call_counter, &func_name, called_func, num_args)
            }
            _ => {
                panic!("Unable to match line: {}", line);
            }
        };
        asm.push_str(&format!("//{}\n", &line));
        asm.push_str(&instructions);
    }

    return asm;
}

fn write_call(call_counter: &u32, func_name: &str, called_func: &str, num_args: usize) -> String {
    let mut asm = String::new();
    let ret_addr = format!("{}$ret{}", func_name, call_counter);

    // Save caller function frame onto stack
    let push_ret_addr = format!("@{}\nD=A\n{}\n", ret_addr, push_to_stack());
    let lcl = format!("@LCL\nD=M\n{}\n", push_to_stack());
    let arg = format!("@ARG\nD=M\n{}\n", push_to_stack());
    let this = format!("@THIS\nD=M\n{}\n", push_to_stack());
    let that = format!("@THAT\nD=M\n{}\n", push_to_stack());
    asm.push_str(&push_ret_addr);
    asm.push_str(&lcl);
    asm.push_str(&arg);
    asm.push_str(&this);
    asm.push_str(&that);

    // Init called function frame
    let init_arg = format!("@SP\nD=M\n@5\nD=D-A\n@{num_args}\nD=D-A\n@ARG\nM=D\n");
    let init_lcl = format!("@SP\nD=M\n@LCL\nM=D\n");
    asm.push_str(&init_arg);
    asm.push_str(&init_lcl);

    // Jump to function and generate return addr label
    let jmp_to_func = format!("@{called_func}\n0;JMP\n");
    asm.push_str(&jmp_to_func);
    let ret_addr_label = format!("({ret_addr})\n");
    asm.push_str(&ret_addr_label);

    return asm;
}
fn write_return() -> String {
    let mut asm = String::new();

    // tmp variable we set to end of func frame
    let frame = format!("@LCL\nD=M\n@frame\nM=D\n");
    asm.push_str(&frame);
    // stores the return addr in a tmp var b/c setting return_val might override it
    let ret_addr = format!("@5\nD=A\n@frame\nA=M-D\nD=M\n@retAddr\nM=D\n");
    asm.push_str(&ret_addr);
    // Sets first arg that stack ptr is at to the return value
    let return_val = format!("{}@ARG\nA=M\nM=D\n", pop_from_stack());
    asm.push_str(&return_val);
    // Repositions stack ptr to point to before the function frame at first arg
    // This way we don't need to pop all the frame data/args we pushed
    let set_sp = format!("@ARG\nD=M+1\n@SP\nM=D\n");
    asm.push_str(&set_sp);

    // Get back saved values that we pushed to stack before calling func
    let that = format!("@1\nD=A\n@frame\nA=M-D\nD=M\n@THAT\nM=D\n");
    let this = format!("@2\nD=A\n@frame\nA=M-D\nD=M\n@THIS\nM=D\n");
    let arg = format!("@3\nD=A\n@frame\nA=M-D\nD=M\n@ARG\nM=D\n");
    let lcl = format!("@4\nD=A\n@frame\nA=M-D\nD=M\n@LCL\nM=D\n");
    asm.push_str(&that);
    asm.push_str(&this);
    asm.push_str(&arg);
    asm.push_str(&lcl);

    // jump back
    let return_jmp = format!("@retAddr\nA=M\n0;JMP\n");
    asm.push_str(&return_jmp);

    return asm;
}

fn write_label(label: &str, func_name: &str) -> String {
    format!("({}${})\n", func_name, label)
}
fn write_goto(label: &str, func_name: &str) -> String {
    format!("@{}${}\n0;JMP\n", func_name, label)
}
fn write_if_goto(label: &str, func_name: &str) -> String {
    // jump if val on stack is -1 (true)
    format!("{}\n@{}${}\nD;JNE\n", pop_from_stack(), func_name, label)
}
fn write_function(func_name: &str, num_vars: usize) -> String {
    let mut asm = format!("({func_name})\n");
    // n vars times push 0 which initialize thme
    for _ in 0..num_vars {
        let init = format!("D=0\n{}\n", push_to_stack());
        asm.push_str(&init);
    }

    return asm;
}

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
fn set_compare_result(file_name: &str, count: u32) -> String {
    format!(
        "D=0\n@{file_name}.FALSE{count}\n0;JMP\n({file_name}.TRUE{count})\nD=-1\n({file_name}.FALSE{count})\n"
    )
}

fn greater_than(file_name: &str, count: u32) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_jmp = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n@{file_name}.TRUE{count}\nD;JGT\n");
    asm.push_str(&pop_and_jmp);
    let comparison = set_compare_result(file_name, count);
    asm.push_str(&comparison);
    asm.push_str(&push_to_stack());
    return asm;
}

fn less_than(file_name: &str, count: u32) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_jmp = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n@{file_name}.TRUE{count}\nD;JLT\n");
    asm.push_str(&pop_and_jmp);
    let comparison = set_compare_result(file_name, count);
    asm.push_str(&comparison);
    asm.push_str(&push_to_stack());
    return asm;
}

fn equal_to(file_name: &str, count: u32) -> String {
    let mut asm = String::new();
    asm.push_str(&pop_from_stack());
    let pop_and_jmp = format!("@SP\nM=M-1\n@SP\nA=M\nD=M-D\n@{file_name}.TRUE{count}\nD;JEQ\n");
    asm.push_str(&pop_and_jmp);
    let comparison = set_compare_result(file_name, count);
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
