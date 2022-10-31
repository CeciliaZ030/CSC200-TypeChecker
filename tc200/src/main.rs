use std::collections::HashMap;
use std::env::args;
use std::io::prelude::*;
use std::fs::File;
use tc200::*;

fn main() {

    println!("\n\n---------- tc200 Program Started ----------");

    // get file from command line argument (will only accept one argument)
    let file_name = args().nth(1).unwrap().to_string();
    let mut file = File::open("./src/files/".to_string() + &file_name).expect("Unable to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read the file");

    println!("Expression: {}", content);

	let parser: Vec<Token> = tokenize(content);
    let ast = parse(&parser);
    let tnv: HashMap<String, Type> = HashMap::new();
    let result = tc(ast, &tnv);

    println!("The type is {{ {:?} }}.", result);
}
