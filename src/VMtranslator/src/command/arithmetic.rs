use crate::command::Command;

pub(crate) struct CArithmeticAdd;
impl Command for CArithmeticAdd {
    fn write(&self) -> Vec<String> {
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

        return vec.to_vec();
    }
}

pub(crate) struct CArithmeticSub;
impl Command for CArithmeticSub {
    fn write(&self) -> Vec<String> {
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

        return vec.to_vec();
    }
}

pub(crate) struct CArithmeticNeg;
impl Command for CArithmeticNeg {
    fn write(&self) -> Vec<String> {
        let vec = [
            String::from("// neg"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=-M"),

            String::from("@SP"),
            String::from("M=M+1")
        ];

        return vec.to_vec();
    }
}