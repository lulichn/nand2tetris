use crate::command::Command;

pub(crate) struct CArithmeticAnd;
impl Command for CArithmeticAnd {
    fn write(&self) -> Vec<String> {
        vec![
            String::from("// and"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M&D"),

            String::from("@SP"),
            String::from("M=M+1"),
        ]
    }
}

pub(crate) struct CArithmeticOr;
impl Command for CArithmeticOr {
    fn write(&self) -> Vec<String> {
        vec![
            String::from("// or"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M|D"),

            String::from("@SP"),
            String::from("M=M+1"),
        ]
    }
}

pub(crate) struct CArithmeticNot;
impl Command for CArithmeticNot {
    fn write(&self) -> Vec<String> {
        vec![
            String::from("// not"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=!M"),

            String::from("@SP"),
            String::from("M=M+1"),
        ]
    }
}
