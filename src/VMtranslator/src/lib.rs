
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

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
    let mut reader = BufReader::new(in_file);

    let lines = analyze(&mut reader)?;

    // out
    let stem = Path::new(&config.input).file_stem().unwrap().to_str().unwrap();
    let output = format!("{}{}", stem, ".asm");

    let out_file = File::create(&output)?;
    let mut writer = BufWriter::new(out_file);

    let mut id = 0;
    for line in lines {
        let codes = line.write(id);
        id = id + 1;
        for code in codes {
            writer.write(format!("{}\n", code).as_bytes()).unwrap();
        }
    }

    Ok(())
}

fn analyze(reader: &mut BufReader<File>) -> Result<Vec<Box<dyn Command>>, Box<dyn Error>> {
    let mut lines: Vec<Box<dyn Command>> = Vec::new();

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

        let vec: Vec<&str> = line.split_whitespace().collect();
        let boxed = make_command(vec);
        lines.push(boxed);
    }

    return Ok(lines);
}

pub fn base_address(str_value: &str) -> &str {
    match str_value {
        "local"    => "@LCL",
        "argument" => "@ARG",
        "this"     => "@THIS",
        "that"    => "@THAT",
        _ => unreachable!()
    }
}

trait Command {
    fn write(&self, id: i32) -> Vec<String> ;
}

fn make_command(vec: Vec<&str>) -> Box<dyn Command> {
    match vec[0] {
        "add"  => Box::new(CArithmeticAdd),
        "sub"  => Box::new(CArithmeticSub),
        "neg"  => Box::new(CArithmeticNeg),
        "eq"   => Box::new(CArithmeticEq),
        "gt"   => Box::new(CArithmeticGt),
        "lt"   => Box::new(CArithmeticLt),
        "and"  => Box::new(CArithmeticAnd),
        "or"   => Box::new(CArithmeticOr),
        "not"  => Box::new(CArithmeticNot),
        "push" => Box::new(CPush { arg1: vec[1].to_string(), arg2: vec[2].to_string() }),
        "pop"  => Box::new(CPop { arg1: vec[1].to_string(), arg2: vec[2].to_string() }),
        _ => unreachable!()
    }
}

struct CPush {
    arg1: String,
    arg2: String,
}
impl Command for CPush {
    fn write(&self, _: i32) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(format!("// push {} {}", self.arg1, self.arg2));

        let mut add = match self.arg1.as_str() {
            "constant" => vec![
                format!("@{}", self.arg2),
                String::from("D=A")
            ],
            "local"|"argument"|"this"|"that" => vec![
                format!("@{}", self.arg2),
                String::from("D=A"),
                base_address(self.arg1.as_str()).to_string(),
                String::from("A=M+D"),
                String::from("D=M"),
            ],
            "pointer" => vec![
                format!("@{}", 3 + self.arg2.parse::<i32>().unwrap()),
                String::from("D=M"),
            ],
            "temp" => vec![
                format!("@{}", 5 + self.arg2.parse::<i32>().unwrap()),
                String::from("D=M"),
            ],
            "static" => vec![
            ],
            _ => unimplemented!()
        };

        vec.append(&mut add);

        vec.append(&mut vec![
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1")
        ]);

        println!("{:?}", vec);
        return vec;
    }
}

struct CPop {
    arg1: String,
    arg2: String,
}
impl Command for CPop {
    fn write(&self, _: i32) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push(format!("// pop {} {}", self.arg1, self.arg2));

        let mut add = match self.arg1.as_str() {
            "local"|"argument"|"this"|"that" => vec![
                format!("@{}", self.arg2),
                String::from("D=A"),
                base_address(self.arg1.as_str()).to_string(),
                String::from("D=M+D"),

                // temp
                String::from("@R13"),
                String::from("M=D"),

                // Pop
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),

                // store
                String::from("@R13"),
                String::from("A=M"),
                String::from("M=D"),
            ],
            "pointer" => vec![
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),

                format!("@{}", 3 + self.arg2.parse::<i32>().unwrap()),
                String::from("M=D"),
            ],
            "temp" => vec![
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),

                format!("@{}", 5 + self.arg2.parse::<i32>().unwrap()),
                String::from("M=D"),
            ],
            "static" => vec![
            ],
            _ => unimplemented!()
        };

        vec.append(&mut add);

        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticAdd;
impl Command for CArithmeticAdd {
    fn write(&self, _: i32) -> Vec<String> {
        let vec = [
            String::from("// add"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M+D"),

            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticSub;
impl Command for CArithmeticSub {
    fn write(&self, _: i32) -> Vec<String> {
        let vec = [
            String::from("// sub"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M-D"),

            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticNeg;
impl Command for CArithmeticNeg {
    fn write(&self, _: i32) -> Vec<String> {
        let vec = [
            String::from("// neg"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=-M"),

            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticEq;
impl Command for CArithmeticEq {
    fn write(&self, id: i32) -> Vec<String> {
        let vec = [
            String::from("// eq"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M-D"),

            format!("@EQ_{}", id),
            String::from("D;JEQ"),

            String::from("@SP"),
            String::from("A=M"),
            String::from("M=0"),

            format!("@EQ_END_{}", id),
            String::from("0;JMP"),

            format!("(EQ_{})", id),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=-1"),

            format!("(EQ_END_{})", id),
            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticGt;
impl Command for CArithmeticGt {
    fn write(&self, id: i32) -> Vec<String> {
        let vec = [
            String::from("// gt"),
            // sp-1 : y
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            // sp-2 : x
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M-D"),

            format!("@GT_{}", id),
            String::from("D;JGT"),

            String::from("@SP"),
            String::from("A=M"),
            String::from("M=0"),

            format!("@GT_END_{}", id),
            String::from("0;JMP"),

            format!("(GT_{})", id),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=-1"),

            format!("(GT_END_{})", id),
            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticLt;
impl Command for CArithmeticLt {
    fn write(&self, id: i32) -> Vec<String> {
        let vec = [
            String::from("// lt"),
            // sp-1 : y
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            // sp-2 : x
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M-D"),

            format!("@LT_{}", id),
            String::from("D;JLT"),

            String::from("@SP"),
            String::from("A=M"),
            String::from("M=0"),

            format!("@LT_END_{}", id),
            String::from("0;JMP"),

            format!("(LT_{})", id),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=-1"),

            format!("(LT_END_{})", id),
            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticAnd;
impl Command for CArithmeticAnd {
    fn write(&self, _: i32) -> Vec<String> {
        let vec = [
            String::from("// and"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M&D"),

            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticOr;
impl Command for CArithmeticOr {
    fn write(&self, _: i32) -> Vec<String> {
        let vec = [
            String::from("// or"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M|D"),

            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}

struct CArithmeticNot;
impl Command for CArithmeticNot {
    fn write(&self, _: i32) -> Vec<String> {
        let vec = [
            String::from("// not"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=!M"),

            String::from("@SP"),
            String::from("M=M+1")
        ];
        println!("{:?}", vec);
        return vec.to_vec();
    }
}
