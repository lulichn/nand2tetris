use std::error::Error;
use std::ffi::OsStr;
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

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
    let in_path = Path::new(&config.input);

    if in_path.is_dir() {
        let files = list_files(in_path)?;

        let mut lines: Vec<Box<dyn Command>> = Vec::new();
        lines.push(command::bootstrap());
        for file in files {
            lines.append(&mut read_file(&file)?);
        }

        // out
        let file_name = in_path.dir_name()?;
        write(file_name, lines)?;
    } else {
        let lines = read_file(in_path)?;

        // out
        let file_name = in_path.file_stem().unwrap().to_str().unwrap().to_string();
        write(file_name, lines)?;
    }


    Ok(())
}

fn list_files(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>>{
    let files = path.read_dir()?;
    let vm_paths = files.filter_map(Result::ok)
        .filter(|f: &DirEntry| f.path().extension() == Some(OsStr::new("vm")))
        .map(|x| x.path())
        .collect();

    Ok(vm_paths)
}

fn read_file(path: &Path) -> Result<Vec<Box<dyn Command>>, Box<dyn Error>> {
    let file_name = path.file_stem().unwrap().to_str().unwrap();

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

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
        let boxed = command::make_command(file_name, id, tokens);
        lines.push(boxed);
        id += 1;
    }

    Ok(lines)
}

fn write(file_name: String, lines: Vec<Box<dyn Command>>) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file_name).with_extension("asm");
    let out_file = File::create(path)?;
    let mut writer = BufWriter::new(out_file);

    for line in lines {
        let codes = line.write();
        for code in codes {
            writer.write(format!("{}\n", code).as_bytes()).unwrap();
        }
    }

    Ok(())
}

pub trait PathMixin {
    fn dir_name(&self) -> Result<String, Box<dyn Error>>;
}

impl PathMixin for Path {
    fn dir_name(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.canonicalize()?.file_name().unwrap().to_str().unwrap().to_string())
    }
}