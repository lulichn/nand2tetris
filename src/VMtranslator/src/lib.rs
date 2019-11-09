use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::command::Command;

mod command;

pub struct Config {
    input: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let input = args[1].clone();
        Ok(Config { input })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let in_file = File::open(&config.input)?;
    let file_name = Path::new(&config.input).file_stem().unwrap().to_str().unwrap();

    let mut reader = BufReader::new(in_file);

    let lines = translate(file_name, &mut reader)?;

    // out
    let out_file_name = format!("{}{}", file_name, ".asm");
    let out_file = File::create(&out_file_name)?;
    let mut writer = BufWriter::new(out_file);

    for line in lines {
        let codes = line.write();
        for code in codes {
            writer.write(format!("{}\n", code).as_bytes()).unwrap();
        }
    }

    Ok(())
}

fn translate(file: &str, reader: &mut BufReader<File>) -> Result<Vec<Box<dyn Command>>, Box<dyn Error>> {
    let mut lines: Vec<Box<dyn Command>> = Vec::new();
    let mut id = 0;

    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        let line = buf.clone();
        buf.clear();

        let mut line = line.trim().to_string();
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        line = match line.find("//") {
            Some(index) => line[0..index].to_string(),
            None => line
        };

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let boxed = command::make_command(file, id, tokens);
        lines.push(boxed);
        id += 1;
    }

    return Ok(lines);
}
