use crate::command::Command;

pub(crate) struct CPush {
    pub(crate) file: String,
    pub(crate) arg1: String,
    pub(crate) arg2: String,
}
impl Command for CPush {
    fn write(&self) -> Vec<String> {
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
                format!("@{}.{}", self.file, self.arg2),
                String::from("D=M"),
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


        return vec;
    }
}

pub struct CPop {
    pub(crate) file: String,
    pub(crate) arg1: String,
    pub(crate) arg2: String,
}

impl Command for CPop {
    fn write(&self) -> Vec<String> {
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
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),

                format!("@{}.{}", self.file, self.arg2),
                String::from("M=D"),
            ],
            _ => unimplemented!()
        };

        vec.append(&mut add);


        return vec.to_vec();
    }
}

pub fn base_address(str_value: &str) -> &str {
    match str_value {
        "local"    => "@LCL",
        "argument" => "@ARG",
        "this"     => "@THIS",
        "that"     => "@THAT",
        _ => unreachable!()
    }
}