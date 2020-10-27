#![allow(dead_code)]

pub mod error;
pub mod scanner;
pub mod misc;
pub mod token;

use std::env;
use std::fs;
//use std::io;
use std::io::{self, Write};

fn run(source: &str) {
    //print!("{}", source);
    //io::stdout().flush().unwrap();
    let mut scanner: scanner::Scanner = scanner::Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token.to_string());
    }
}


fn run_file(filename: &str) {
    let file = fs::read_to_string(filename).expect("Can't read file.");
    run(&file);
}

fn run_prompt() {
    let mut line: String;
    loop {
        line = "".to_string();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).expect("Fail to read line");
        if line.is_empty() {
            break;
        }
        run(&line);
    }
}

fn main(){

    let argv: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let argc = argv.len();
    if argc > 2 {
        println!("Usage: nill [script]");
        //return 2
    } else if argc == 2 {
        run_file(&argv[1]);
    } else {
        run_prompt();
    }
}
