use crate::command::Command;

pub(crate) struct Label {
    pub(crate) file: String,
    pub(crate) arg1: String,
}
impl Command for Label {
    fn write(&self) -> Vec<String> {
        let vec = [
            format!("({})", self.arg1.as_str())
        ];

        return vec.to_vec();
    }
}

pub(crate) struct Goto {
    pub(crate) file: String,
    pub(crate) arg1: String,
}
impl Command for Goto {
    fn write(&self) -> Vec<String> {
        let vec = [
            // Pop
            format!("@{}", self.arg1.as_str()),
            String::from("0;JMP"),
        ];

        return vec.to_vec();
    }
}

pub(crate) struct IfGoto {
    pub(crate) file: String,
    pub(crate) arg1: String,
}
impl Command for IfGoto {
    fn write(&self) -> Vec<String> {
        let vec = [
            // Pop
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            format!("@{}", self.arg1.as_str()),
            String::from("D;JNE"),
        ];

        return vec.to_vec();
    }
}