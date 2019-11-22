use crate::command::Command;

pub(crate) struct Function {
    pub(crate) file: String,
    pub(crate) arg1: String,
    pub(crate) arg2: String,
}
impl Command for Function {
    fn write(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.append(&mut vec![
            format!("// function {} {}", self.arg1.as_str(), self.arg2.as_str()),
            format!("({})", self.arg1.as_str())
        ]);

        let k = self.arg2.parse::<i32>().unwrap();
        for x in 0..k {
            vec.append(&mut vec![
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=0"),
                String::from("@SP"),
                String::from("M=M+1"),
            ]);
        }

        return vec.to_vec();
    }
}

pub(crate) struct Call {
    pub(crate) file: String,
    pub(crate) arg1: String,
    pub(crate) arg2: String,
}
impl Command for Call {
    fn write(&self) -> Vec<String> {
        let vec = [
            format!("({})", self.arg1.as_str())
        ];

        return vec.to_vec();
    }
}

pub(crate) struct Return {
    pub(crate) file: String,
}
impl Command for Return {
    fn write(&self) -> Vec<String> {
        let vec = [
            String::from("// return"),

            // FRAME: Address
            String::from("@LCL"),
            String::from("D=M"),
            String::from("@R13"),
            String::from("M=D"),

            // RET: Address
            String::from("@5"),
            String::from("A=D-A"),
            String::from("D=M"),
            String::from("@R14"),
            String::from("M=D"),

            // *ARG = pop()
            String::from("@ARG"),
            String::from("D=M"),
            String::from("@R15"),
            String::from("M=D"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@15"),
            String::from("A=M"),
            String::from("M=D"),

            // SP = ARG + 1
            String::from("@ARG"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("M=D+1"),

            // THAT
            String::from("@R13"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@THAT"),
            String::from("M=D"),

            // THIS
            String::from("@R13"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@THIS"),
            String::from("M=D"),

            // ARG
            String::from("@R13"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@ARG"),
            String::from("M=D"),

            // LCL
            String::from("@R13"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@LCL"),
            String::from("M=D"),

            // goto RET
            String::from("@R14"),
            String::from("A=M"),
            String::from("0;JMP"),
        ];

        return vec.to_vec();
    }
}