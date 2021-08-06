mod lexer;
pub mod parser;
mod error_handle;

use std::fs;

use clap::{App, Arg};
use lexer::Lexer;

fn main() {
    let matches = App::new("DecisionTree")
        .version("1.0.0")
        .author("Sullivan B")
        .about("Scripting language to define decision trees")
        .arg(Arg::new("filename")
            .index(1)
            .takes_value(true)
            .about("Location of the target file")
            .required(true)
        ).get_matches();

    let filename = matches.value_of("filename").unwrap();

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            println!("File {} doesn't exist, double check the location/name and try again.", filename);
            return;
        }
    };

    let mut lexer = Lexer::new(contents.as_str());

    while let Some(token) = lexer.next() {
        println!("{}", token.to_string());
    }
}
