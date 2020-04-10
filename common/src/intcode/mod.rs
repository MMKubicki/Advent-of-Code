mod computer;
pub mod instruction;

use crate::intcode::instruction::{default_instruction_appy, Instruction};

pub use computer::Computer;
pub use computer::ComputerState;

pub type Code = usize;

pub fn create_computer(level: Level, memory: Vec<usize>) -> Computer {
    let mut c = create_empty_computer(level);
    c.set_memory(memory);
    c
}

pub fn create_empty_computer(level: Level) -> Computer {
    let mut instructions = Vec::with_capacity(1);
    instructions.push(Instruction::new(99, end_instruction));

    let level = level as usize;

    if level > 0 {
        instructions.reserve(2);
        instructions.push(Instruction::new(1, add_instruction));
        instructions.push(Instruction::new(2, mul_instruction));
    }

    Computer::new_with_instructions(Vec::new(), instructions)
}

#[non_exhaustive]
pub enum Level {
    None = 0,
    Day02 = 1,
}

fn add_instruction(computer: &mut Computer, address: usize) {
    default_instruction_appy(computer, address, |a, b| a + b)
}

fn mul_instruction(computer: &mut Computer, address: usize) {
    default_instruction_appy(computer, address, |a, b| a * b)
}

fn end_instruction(computer: &mut Computer, _: usize) {
    computer.set_status(ComputerState::Finished)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computer_test1() {
        let mut c = create_computer(Level::Day02, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(c.run_until_end(), ComputerState::Finished);
        assert_eq!(c.get(0), Some(3500));
    }

    #[test]
    fn computer_test2() {
        let mut c = create_computer(Level::Day02, vec![1, 0, 0, 0, 99]);

        assert_eq!(c.run_until_end(), ComputerState::Finished);
        assert_eq!(c.get(0), Some(2));
    }

    #[test]
    fn computer_test3() {
        let mut c = create_computer(Level::Day02, vec![2, 3, 0, 3, 99]);

        assert_eq!(c.run_until_end(), ComputerState::Finished);
        assert_eq!(c.get(3), Some(6));
    }

    #[test]
    fn computer_test4() {
        let mut c = create_computer(Level::Day02, vec![2, 4, 4, 5, 99, 0]);

        assert_eq!(c.run_until_end(), ComputerState::Finished);
        assert_eq!(c.get(5), Some(9801));
    }

    #[test]
    fn computer_test5() {
        let mut c = create_computer(Level::Day02, vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);

        assert_eq!(c.run_until_end(), ComputerState::Finished);
        assert_eq!(c.get(0), Some(30));
    }
}
