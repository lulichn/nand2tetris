extern crate vm_translator;

use std::{env, process};
use vm_translator::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    vm_translator::run(config).unwrap();
}
