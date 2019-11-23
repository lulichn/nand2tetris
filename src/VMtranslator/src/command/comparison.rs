use crate::command::Command;

pub(crate) enum ComparisonType {
    Eq,
    Gt,
    Lt,
}

impl ComparisonType {
    fn as_str(&self) -> &str{
        match self {
            ComparisonType::Eq => "EQ",
            ComparisonType::Gt => "GT",
            ComparisonType::Lt => "LT",
        }
    }

    fn mnemonic(&self) -> &str {
        match self {
            ComparisonType::Eq => "JEQ",
            ComparisonType::Gt => "JGT",
            ComparisonType::Lt => "JLT",
        }
    }
}

pub(crate) struct CArithmeticComparison {
    pub(crate) t: ComparisonType,
    pub(crate) id: i32,
}
impl Command for CArithmeticComparison {
    fn write(&self) -> Vec<String> {
        vec![
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),

            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M-D"),

            format!("@{}_{}", self.t.as_str(), self.id),
            format!("D;{}", self.t.mnemonic()),

            String::from("@SP"),
            String::from("A=M"),
            String::from("M=0"),

            format!("@{}_END_{}", self.t.as_str(), self.id),
            String::from("0;JMP"),

            format!("({}_{})", self.t.as_str(), self.id),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=-1"),

            format!("({}_END_{})", self.t.as_str(), self.id),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
    }
}
