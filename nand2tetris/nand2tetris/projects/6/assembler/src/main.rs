use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Can't read file");
    // let symbol_table = hashmap;
    // first_pass(&mut symbol_table, &contents);
    // do two passes, first one populates symbol table, second one translates ops into machine code
    parse_asm(&contents);
}

// fn first_pass(){}
// fn second_pass(){}
// fn translate_instruction(instruction: &AsmInstruction){
//  match instruction {
//
//  }
// }
// fn resolve_symbol(){}
//

enum AsmInstruction {
    WriteM,
}

fn parse_asm(contents: &String) {
    let mut line_num = 0;
    for line in contents.lines() {
        if line.starts_with("\\") {
            println!("It's a comment");
        } else if line.starts_with("(") {
            println!("It's a symbol label");
        } else if line.starts_with("@") {
            println!("Load to A reg");
        } else if line.starts_with("D") {
            println!("Load to D");
        } else if line.starts_with("M") {
            println!("Write to memory M value");
        }
    }
}
