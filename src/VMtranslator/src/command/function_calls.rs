use crate::command::Command;

pub(crate) struct Bootstrap;
impl Command for Bootstrap {
    fn write(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.append(&mut vec![
            String::from("// bootstrap"),
            String::from("@256"),
            String::from("D=A"),
            String::from("@SP"),
            String::from("M=D"),
        ]);

        let call = CCall { file: String::from("bootstrap"), arg1: String::from("Sys.init"), arg2: String::from("0"), id: 0 };
        vec.append(&mut call.write());

        vec
    }
}

pub(crate) struct CCall {
    pub(crate) file: String,
    pub(crate) arg1: String,
    pub(crate) arg2: String,
    pub(crate) id: i32,
}
impl Command for CCall {
    fn write(&self) -> Vec<String> {
        let vec = [
            format!("// call {} {}", self.arg1, self.arg2),

            // push return-address
            format!("@{}_{}", self.file, self.id),
            String::from("D=A"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),

            // push LCL
            String::from("@LCL"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),

            // push ARG
            String::from("@ARG"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),

            // push THIS
            String::from("@THIS"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),

            // push THAT
            String::from("@THAT"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),

            // ARG = SP - n - 5
            String::from("@SP"),
            String::from("D=M"),
            format!("@{}", 5 + self.arg2.parse::<i32>().unwrap()),
            String::from("D=D-A"),
            String::from("@ARG"),
            String::from("M=D"),

            // LCL = SP
            String::from("@SP"),
            String::from("D=M"),
            String::from("@LCL"),
            String::from("M=D"),

            // goto f
            format!("@{}", self.arg1.as_str()),
            String::from("0;JMP"),

            // return-address
            format!("({}_{})", self.file, self.id),
        ];

        return vec.to_vec();
    }
}

pub(crate) struct CFunction {
    pub(crate) arg1: String,
    pub(crate) arg2: String,
}
impl Command for CFunction {
    fn write(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        vec.append(&mut vec![
            format!("// function {} {}", self.arg1.as_str(), self.arg2.as_str()),
            format!("({})", self.arg1.as_str())
        ]);

        let k = self.arg2.parse::<i32>().unwrap();
        for _ in 0..k {
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

pub(crate) struct CReturn;
impl Command for CReturn {
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