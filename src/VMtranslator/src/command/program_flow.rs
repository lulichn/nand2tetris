use crate::command::Command;

pub(crate) struct CLabel {
    pub(crate) file: String,
    pub(crate) arg1: String,
}
impl Command for CLabel {
    fn write(&self) -> Vec<String> {
        let vec = [
            format!("({}${})", self.file, self.arg1)
        ];

        return vec.to_vec();
    }
}

pub(crate) struct CGoto {
    pub(crate) file: String,
    pub(crate) arg1: String,
}
impl Command for CGoto {
    fn write(&self) -> Vec<String> {
        let vec = [
            // Pop
            format!("@{}${}", self.file, self.arg1),
            String::from("0;JMP"),
        ];

        return vec.to_vec();
    }
}

pub(crate) struct CIfGoto {
    pub(crate) file: String,
    pub(crate) arg1: String,
}
impl Command for CIfGoto {
    fn write(&self) -> Vec<String> {
        let vec = [
            // Pop
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            format!("@{}${}", self.file, self.arg1),
            String::from("D;JNE"),
        ];

        return vec.to_vec();
    }
}