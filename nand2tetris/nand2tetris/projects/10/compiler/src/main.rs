// #![allow(warnings)]
use std::env;
use std::fs;

mod parser;
use crate::parser::NonTerminalElement;
use crate::parser::TokenParser;
use crate::parser::handle_class;

mod lexer;
use crate::lexer::Lexer;
use crate::lexer::tokenize;

mod code_gen;
use crate::code_gen::code_gen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if file_path.contains(".jack") {
        // let file_name = file_path.split("/").last().unwrap().replace(".jack", "");
        let contents = fs::read_to_string(file_path).expect("Can't read file!");
        let tree = create_program_tree(&contents);
        code_gen(tree);
        // let xml = tree.to_xml();
        // let xml_path = file_path.replace(".jack", "test.xml");
        // let _ = fs::write(xml_path, xml);
    } else {
        let paths = fs::read_dir(file_path).expect("Couldn't read directory");
        for path in paths {
            if let Ok(path) = path {
                if let Ok(file_name) = path.file_name().into_string() {
                    if file_name.contains(".jack") {
                        let contents = fs::read_to_string(path.path()).expect("Can't read file!");
                        let tree = create_program_tree(&contents);
                        code_gen(tree);
                        // let xml = tree.to_xml();
                        // let xml_path = path.path().to_str().unwrap().replace(".jack", "test.xml");
                        // let _ = fs::write(xml_path, xml);
                    }
                }
            }
        }
    }
}

fn create_program_tree(contents: &str) -> NonTerminalElement {
    let ch_vec: Vec<u8> = contents.bytes().collect();
    let mut lexer = Lexer::new(ch_vec);
    tokenize(&mut lexer);
    let mut parser = TokenParser {
        tokens: lexer.tokens,
    };

    return handle_class(&mut parser);
}
