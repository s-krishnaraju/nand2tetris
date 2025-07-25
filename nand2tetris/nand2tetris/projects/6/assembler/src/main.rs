use std::collections::HashMap;
use std::env;
use std::fs;

const PLATFORM_BYTES: usize = 16;

#[derive(Debug)]
struct CInstruction {
    comp: String,
    dest: Option<String>,
    jmp: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_name = match file_path.split("/").last() {
        Some(fname) => fname.replace(".asm", ".hack"),
        None => "no_name.hack".to_string(),
    };

    let contents = fs::read_to_string(file_path).expect("Can't read file!");
    let mut symbol_table = init_symbol_table();
    first_pass(&contents, &mut symbol_table);
    let hack_file: String = parse_asm(&contents, &symbol_table);

    fs::write(format!("./{}", &file_name), hack_file).expect("Can't write file!");
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

fn remove_all_whitespace(s: &str) -> String {
    return s.split_whitespace().collect();
}

fn parse_a_instruction(instruction: &str) -> String {
    let split: Vec<&str> = instruction.split("@").collect();
    if split.len() == 2 {
        return split[1].to_string();
    } else {
        panic!("Error parsing A instruction");
    }
}

fn parse_c_instruction(instruction: &str) -> CInstruction {
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
    // handle (label)
    let mut line_num = 0;
    for line in contents.lines() {
        let instruction = remove_all_whitespace(line);
        if line.is_empty() || instruction.starts_with('/') {
            continue;
        } else if instruction.starts_with("(") {
            let symbol = instruction.replace(&['(', ')'], "");
            symbol_table.insert(symbol, line_num);
        } else {
            line_num += 1;
        }
    }
    // handle @label
    let mut symbol_num = 16;
    for line in contents.lines() {
        let instruction = remove_all_whitespace(line);
        if !line.is_empty() && instruction.starts_with('@') {
            let symbol = parse_a_instruction(&instruction);
            if !symbol.starts_with(|c: char| c.is_numeric()) {
                if !symbol_table.contains_key(&symbol) {
                    symbol_table.insert(symbol, symbol_num);
                    symbol_num += 1;
                }
            }
        }
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

fn handle_comp_instruct(comp: &str) -> [char; 6] {
    return match comp {
        "0" => ['1', '0', '1', '0', '1', '0'],
        "1" => ['1', '1', '1', '1', '1', '1'],
        "-1" => ['1', '1', '1', '0', '1', '0'],
        "D" => ['0', '0', '1', '1', '0', '0'],
        "A" | "M" => ['1', '1', '0', '0', '0', '0'],
        "!D" => ['0', '0', '1', '1', '0', '1'],
        "!A" | "!M" => ['1', '1', '0', '0', '0', '1'],
        "-D" => ['0', '0', '1', '1', '1', '1'],
        "-A" | "-M" => ['1', '1', '0', '0', '1', '1'],
        "D+1" => ['0', '1', '1', '1', '1', '1'],
        "A+1" | "M+1" => ['1', '1', '0', '1', '1', '1'],
        "D-1" => ['0', '0', '1', '1', '1', '0'],
        "A-1" | "M-1" => ['1', '1', '0', '0', '1', '0'],
        "D+A" | "D+M" => ['0', '0', '0', '0', '1', '0'],
        "D-A" | "D-M" => ['0', '1', '0', '0', '1', '1'],
        "A-D" | "M-D" => ['0', '0', '0', '1', '1', '1'],
        "D&A" | "D&M" => ['0', '0', '0', '0', '0', '0'],
        "D|A" | "D|M" => ['0', '1', '0', '1', '0', '1'],
        _ => panic!("Error matching comp instruction"),
    };
}
fn handle_dest_instruct(dest: &str) -> [char; 3] {
    return match dest {
        "M" => ['0', '0', '1'],
        "D" => ['0', '1', '0'],
        "DM" | "MD" => ['0', '1', '1'],
        "A" => ['1', '0', '0'],
        "AM" | "MA" => ['1', '0', '1'],
        "AD" | "DA" => ['1', '1', '0'],
        "ADM" => ['1', '1', '1'],
        _ => panic!("Error matching dest instruction"),
    };
}

fn handle_jmp_instruct(jmp: &str) -> [char; 3] {
    return match jmp {
        "JGT" => ['0', '0', '1'],
        "JEQ" => ['0', '1', '0'],
        "JGE" => ['0', '1', '1'],
        "JLT" => ['1', '0', '0'],
        "JNE" => ['1', '0', '1'],
        "JLE" => ['1', '1', '0'],
        "JMP" => ['1', '1', '1'],
        _ => panic!("Error matching jmp instruction"),
    };
}
fn handle_c_instruction(instruct: &CInstruction) -> [char; PLATFORM_BYTES] {
    let placeholder = ['1'; 3];
    let a = if instruct.comp.contains("M") {
        ['1']
    } else {
        ['0']
    };

    let c = handle_comp_instruct(&instruct.comp);
    let j = match &instruct.jmp {
        Some(s) => handle_jmp_instruct(&s),
        None => ['0'; 3],
    };

    let d = match &instruct.dest {
        Some(s) => handle_dest_instruct(&s),
        None => ['0'; 3],
    };

    let mut result = ['1'; PLATFORM_BYTES];
    result[..3].copy_from_slice(&placeholder);
    result[3..4].copy_from_slice(&a);
    result[4..10].copy_from_slice(&c);
    result[10..13].copy_from_slice(&d);
    result[13..16].copy_from_slice(&j);
    return result;
}

fn decimal_to_binary(dec: u32) -> [char; PLATFORM_BYTES] {
    const TWO: u32 = 2;
    let mut b = ['0'; PLATFORM_BYTES];
    let mut num = dec;
    while num > 0 {
        let mut n = 0;
        while TWO.pow(n + 1) <= num {
            n += 1;
        }
        num = num - TWO.pow(n);
        if n < b.len() as u32 {
            let idx = PLATFORM_BYTES - 1 - n as usize;
            b[idx] = '1';
        }
    }
    return b;
}

fn parse_asm(contents: &str, symbol_table: &HashMap<String, u32>) -> String {
    let mut hack_file: String = String::new();
    for line in contents.lines() {
        let instruction = remove_all_whitespace(line);

        if line.is_empty() || instruction.starts_with('/') || instruction.starts_with('(') {
            continue;
        }

        let hack_line: String = if instruction.starts_with('@') {
            let symbol = parse_a_instruction(&instruction);
            let addr = get_address(&symbol, symbol_table);
            let addr = decimal_to_binary(addr);
            addr.iter().collect()
        } else {
            let c_instruct = parse_c_instruction(&instruction);
            let c_instruct = handle_c_instruction(&c_instruct);
            c_instruct.iter().collect()
        };

        hack_file.push_str(&hack_line);
        hack_file.push_str("\n");
    }

    return hack_file;
}
