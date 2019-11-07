#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::borrow::BorrowMut;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use regex::Regex;

lazy_static! {
    static ref BUILDIN: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("R0", 0); m.insert("R1", 1); m.insert("R2", 2); m.insert("R3", 3); m.insert("R4", 4); m.insert("R5", 5);
        m.insert("R6", 6); m.insert("R7", 7); m.insert("R8", 8); m.insert("R9", 9); m.insert("R10", 10); m.insert("R11", 11);
        m.insert("R12", 12); m.insert("R13", 13); m.insert("R14", 14); m.insert("R15", 15);
        m.insert("SP", 0); m.insert("LCL", 1); m.insert("ARG", 2); m.insert("THIS", 3); m.insert("THAT", 4);
        m.insert("SCREEN", 16384); m.insert("KBD", 24576);
        m
    };
}

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

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let in_file = File::open(&config.input)?;
    let mut reader = BufReader::new(in_file);

    let lines = analyze(reader.borrow_mut())?;
//    println!("{:?}", lines);

    let table = make_symbol_table(&lines);
//    println!("symbols: {:?}", table);

    let bins = code(&lines, table);
//    println!("{:?}", bins);

    let stem = Path::new(&config.input).file_stem().unwrap().to_str().unwrap();
    let output = format!("{}{}", stem, ".hack");

    let out_file = File::create(&output)?;
    let mut writer = BufWriter::new(out_file);
    for bin in bins {
        writer.write(format!("{}\n", bin).as_bytes()).unwrap();
    }
    Ok(())
}

fn analyze(reader: &mut BufReader<File>) -> Result<Vec<String>, Box<Error>> {
    let mut lines: Vec<String> = Vec::new();

    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        let line = buf.clone();
        buf.clear();

        let mut line = line.trim().to_string();
        if line.starts_with("//") || line.is_empty() {
            continue;
        }
        line.retain(|c| c != ' ');
        line = match line.find("//") {
            Some(index) => line[0..index].to_string(),
            None => line
        };
        lines.push(line);
    }

    return Ok(lines);
}

fn make_symbol_table(lines: &Vec<String>) -> HashMap<String, i32, RandomState> {
    let mut symbol_table = HashMap::new();
    let mut counter = 0;

    for line in lines {
        match command_type(line) {
            Command::L {symbol} => symbol_table.insert(symbol, counter),
            _ => {
                counter += 1;
                None
            }
        };
    }

    return symbol_table;
}

fn code(lines: &Vec<String>, table: HashMap<String, i32, RandomState>) -> Vec<String> {
    let mut vars_table = HashMap::new();
    let mut vars_index = 16;

    let mut bins: Vec<String> = Vec::new();

    for line in lines {
        let bin = match command_type(line) {
            Command::ADir {value} =>
                format!("0{:0>15b}", value),
            Command::A {symbol} => {
                let address = (table.get(&symbol).or(vars_table.get(&symbol))).or(BUILDIN.get(symbol.as_str()));

                match address {
                    Some(addr) => format!("0{:0>15b}", addr),
                    None => {
                        vars_table.insert(symbol, vars_index);
                        let code = format!("0{:0>15b}", vars_index);
                        vars_index += 1;
                        code
                    }
                }
            },
            Command::C {dest, comp, jump} => {
                format!("{}{}{}{}",
                        "111",
                        command_comp(comp),
                        command_dest(dest.unwrap_or(String::default())),
                        command_jump(jump.unwrap_or(String::default())))
            },
            Command::L {symbol: _} => "".to_string(),
        };

        if !bin.is_empty() {
            bins.push(bin);
        }
    }

    return bins;
}

pub fn command_type(str: &String) -> Command {
    match str {
        ref it if it.starts_with("@") => {
            let re = Regex::new(r"^@(\d+)$").unwrap();
            match re.captures(str) {
                Some(value) => {
                    let str = value.get(1).unwrap().as_str().to_string();
                    Command::new_a_dir(&str)
                },
                None => Command::new_a(it)
            }
        },
        ref it if it.starts_with("(") => Command::new_l(it),
        ref it => Command::new_c(it),
    }
}

#[derive(Debug)]
pub enum Command {
    ADir { value: u32 },
    A { symbol: String },
    C { dest: Option<String>, comp: String, jump: Option<String> },
    L { symbol: String },
}

impl Command {
    pub fn new_a_dir(str: &String) -> Command {
        Command::ADir { value: str.parse::<u32>().unwrap() }
    }

    pub fn new_a(str: &String) -> Command {
        let mut t = str.clone();
        t.retain(|c| c != '@');
        Command::A{ symbol: t }
    }

    pub fn new_c(str: &String) -> Command {
        let mut split: Vec<&str> = str.split(';').collect();

        let mut dest: Option<String> = None;
        let comp: String;
        let mut jump: Option<String> = None;

        if split.len() == 2 {
            jump = Some(split[1].to_string());
        }

        split = split[0].split('=').collect();
        if split.len() == 2 {
            dest = Some(split[0].to_string());
            comp = split[1].to_string();
        } else {
            comp = split[0].to_string();
        }

        Command::C { dest, comp, jump }
    }

    pub fn new_l(str: &String) -> Command {
        let size = str.len() - 1;
        let t = str[1..size].to_string();
        Command::L{ symbol: t }
    }
}

fn command_dest(str: String) -> String {
    match &*str {
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => "000",
    }.to_string()
}

fn command_jump(str: String) -> String {
    match &*str.to_uppercase() {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "000",
    }.to_string()
}

fn command_comp(str: String) -> String {
    match &*str.to_uppercase() {
        // a = 0
        "0"   => "0101010",
        "1"   => "0111111",
        "-1"  => "0111010",
        "D"   => "0001100",
        "A"   => "0110000", "M"   => "1110000",
        "!D"  => "0001101",
        "!A"  => "0110001", "!M"  => "1110001",
        "-D"  => "0001111",
        "-A"  => "0110011", "-M"  => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111", "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010", "M-1" => "1110010",
        "D+A" => "0000010", "D+M" => "1000010",
        "D-A" => "0010011", "D-M" => "1010011",
        "A-D" => "0000111", "M-D" => "1000111",
        "D&A" => "0000000", "D&M" => "1000000",
        "D|A" => "0010101", "D|M" => "1010101",
        _ => "",
    }.to_string()
}
