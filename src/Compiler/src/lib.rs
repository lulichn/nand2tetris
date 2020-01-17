use std::error::Error;
use std::ffi::OsStr;
use std::fs::{DirEntry, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use crate::token::{Tokens};
use crate::parser::Parser;

pub mod token;
pub mod parser;

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
    let in_path = Path::new(&config.input);

    if in_path.is_dir() {
        let files = list_files(in_path)?;

        for file in files {
            let tokens = read_file(&file)?;
            let file_name = file.file_stem().unwrap().to_str().unwrap().to_string();
            write_tokens(format!("{}T", file_name), &tokens)?;
        }
    } else {
        let tokens = read_file(in_path)?;
        let file_name = in_path.file_stem().unwrap().to_str().unwrap().to_string();
        write_tokens(format!("{}T", file_name), &tokens)?;

        let mut parser = Parser::new(&tokens);
        let strings = parser.parse();
        write_strings(format!("{}my", file_name), &strings)?;
    }

    Ok(())
}

fn list_files(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let files = path.read_dir()?;
    let vm_paths = files.filter_map(Result::ok)
        .filter(|f: &DirEntry| f.path().extension() == Some(OsStr::new("jack")))
        .map(|x| x.path())
        .collect();

    Ok(vm_paths)
}

fn read_file(path: &Path) -> Result<Vec<Tokens>, Box<dyn Error>> {
//    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;

    let tokens = token::read_file(&buf)?;

    Ok(tokens)
}

fn write_tokens(file_name: String, tokens: &Vec<Tokens>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file_name).with_extension("xml");
    let out_file = File::create(path)?;
    let mut writer = BufWriter::new(out_file);

    writer.write(format!("<tokens>\n").as_bytes()).unwrap();

    for token in tokens {
        let node = token.xml_node();
        writer.write(format!("{}\n", node).as_bytes()).unwrap();
    }

    writer.write(format!("</tokens>\n").as_bytes()).unwrap();

    Ok(())
}

fn write_strings(file_name: String, strings: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file_name).with_extension("xml");
    let out_file = File::create(path)?;
    let mut writer = BufWriter::new(out_file);

    for s in strings {
        writer.write(format!("{}\n", s).as_bytes()).unwrap();
    }

    Ok(())
}

//fn write(file_name: String, lines: Vec<Box<dyn Command>>) -> Result<(), Box<dyn Error>> {
//    let path = Path::new(&file_name).with_extension("asm");
//    let out_file = File::create(path)?;
//    let mut writer = BufWriter::new(out_file);
//
//    for line in lines {
//        let codes = line.write();
//        for code in codes {
//            writer.write(format!("{}\n", code).as_bytes()).unwrap();
//        }
//    }
//
//    Ok(())
//}

pub trait PathMixin {
    fn dir_name(&self) -> Result<String, Box<dyn Error>>;
}

impl PathMixin for Path {
    fn dir_name(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.canonicalize()?.file_name().unwrap().to_str().unwrap().to_string())
    }
}
