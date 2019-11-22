use crate::command::arithmetic::*;
use crate::command::comparison::*;
use crate::command::logical::*;
use crate::command::stack::*;
use crate::command::program_flow::*;

mod stack;
mod arithmetic;
mod logical;
mod comparison;
mod program_flow;

pub trait Command {
    fn write(&self) -> Vec<String> ;
}

pub fn make_command(file: &str, id: i32, tokens: Vec<&str>) -> Box<dyn Command> {
    let command = tokens[0];
    match command {
        "add"     => Box::new(CArithmeticAdd),
        "sub"     => Box::new(CArithmeticSub),
        "neg"     => Box::new(CArithmeticNeg),
        "eq"      => Box::new(CArithmeticComparison{ t: ComparisonType::Eq, id }),
        "gt"      => Box::new(CArithmeticComparison{ t: ComparisonType::Gt, id }),
        "lt"      => Box::new(CArithmeticComparison{ t: ComparisonType::Lt, id }),
        "and"     => Box::new(CArithmeticAnd),
        "or"      => Box::new(CArithmeticOr),
        "not"     => Box::new(CArithmeticNot),
        "push"    => Box::new(CPush { file: file.to_string(), arg1: tokens[1].to_string(), arg2: tokens[2].to_string() }),
        "pop"     => Box::new(CPop { file: file.to_string(), arg1: tokens[1].to_string(), arg2: tokens[2].to_string() }),
        "label"   => Box::new(Label { file: file.to_string(), arg1: tokens[1].to_string() }),
        "goto"    => Box::new(Goto { file: file.to_string(), arg1: tokens[1].to_string() }),
        "if-goto" => Box::new(IfGoto { file: file.to_string(), arg1: tokens[1].to_string() }),
        _ => unreachable!()
    }
}
