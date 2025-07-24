#![allow(warnings)]
use std::collections::HashMap;
use std::env;
use std::fs;
use std::slice::SplitInclusiveMut;
use std::thread::panicking;

#[derive(Debug)]
struct CInstruction {
    comp: String,
    dest: Option<String>,
    jmp: Option<String>,
}

fn init_symbol_table() -> HashMap<String, u32> {
    let mut table: HashMap<String, u32> = HashMap::new();
    table.insert("SCREEN".to_string(), 16384);
    table.insert("KBD".to_string(), 24576);
    table.insert("SP".to_string(), 0);
    table.insert("LCL".to_string(), 1);
    table.insert("ARG".to_string(), 2);
    table.insert("THIS".to_string(), 3);
    table.insert("THAT".to_string(), 4);

    for i in 0..16 {
        let r = format!("R{}", i.to_string());
        table.insert(r, i);
    }

    return table;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut contents = fs::read_to_string(file_path).expect("Can't read file");
    let mut symbol_table = init_symbol_table();
    first_pass(&contents, &mut symbol_table);
    parse_asm(&contents, &symbol_table);
}

fn remove_all_whitespace(s: &str) -> String {
    return s.split_whitespace().collect();
}

fn parse_A_instruction(instruction: &str) -> String {
    let split: Vec<&str> = instruction.split("@").collect();
    if split.len() == 2 {
        return split[1].to_string();
    } else {
        panic!("Error parsing A instruction");
    }
}

fn parse_C_instruction(instruction: &str) -> CInstruction {
    let mut c_instruct = CInstruction {
        comp: String::new(),
        dest: None,
        jmp: None,
    };

    let split_jmp: Vec<&str> = instruction.split(";").collect();
    if split_jmp.len() == 2 {
        c_instruct.jmp = Some(split_jmp[1].to_string());
    } else if split_jmp.len() > 2 {
        panic!("Error parsing C instruction!");
    }

    let split_dest: Vec<&str> = split_jmp[0].split("=").collect();

    if split_dest.len() == 1 {
        c_instruct.comp = split_dest[0].to_string();
    } else if split_dest.len() == 2 {
        c_instruct.dest = Some(split_dest[0].to_string());
        c_instruct.comp = split_dest[1].to_string();
    } else {
        panic!("Error parsing C instruction!");
    }

    return c_instruct;
}

fn first_pass(contents: &str, symbol_table: &mut HashMap<String, u32>) {
    let mut line_num = 0;
    let mut symbol_ram = 16;

    for line in contents.lines() {
        let instruction = remove_all_whitespace(line);
        if (instruction.starts_with('/')) {
            continue;
        } else if (instruction.starts_with('(')) {
            let mut symbol = instruction.replace(&['(', ')'], "");
            symbol_table.insert(symbol, line_num);
            continue;
        } else if (instruction.starts_with('@')) {
            let mut symbol = parse_A_instruction(&instruction);
            if !symbol.starts_with(|c: char| c.is_numeric()) {
                if !symbol_table.contains_key(&symbol) {
                    symbol_table.insert(symbol, symbol_ram);
                    symbol_ram += 1;
                }
            }
        } else {
            let parsed = parse_C_instruction(&instruction);

            dbg!(&parsed);
        }
        line_num += 1;
    }
}

fn get_address(symbol: &str, symbol_table: &HashMap<String, u32>) -> u32 {
    if let Some(symbol_addr) = symbol_table.get(symbol) {
        return symbol_addr.clone();
    } else {
        // otherwise it's num/hardcoded
        match symbol.parse::<u32>() {
            Ok(addr) => {
                return addr;
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}

enum Binary {
    Zero,
    One,
}

fn handle_C_instruction(instruction: CInstruction, symbol_table: &HashMap<String, u32>) {}

fn generate_A_instruct_binary(addr: u32) {
    let b: Vec<Binary> = Vec::from([Binary::Zero]);
}

fn parse_asm(contents: &str, symbol_table: &HashMap<String, u32>) {
    for line in contents.lines() {
        let instruction = remove_all_whitespace(line);

        if (instruction.starts_with('/') || instruction.starts_with('(')) {
            continue;
        } else if (instruction.starts_with('@')) {
            let mut symbol = parse_A_instruction(&instruction);
            let addr = get_address(&symbol, symbol_table);
        } else {
            let parsed = parse_C_instruction(&instruction);
        }
    }
}
