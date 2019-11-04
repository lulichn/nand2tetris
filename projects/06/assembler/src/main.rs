extern crate assembler;
extern crate regex;

use std::{env, process};
use assembler::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    assembler::run(config).unwrap();
}
