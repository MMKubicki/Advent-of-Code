use crate::intcode::computer::ComputerState::Runnable;
use crate::intcode::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Computer {
    main_memory: Vec<usize>,
    extended_memory: Option<HashMap<usize, usize>>,
    instruction_pointer: usize,
    instructions: HashMap<usize, Instruction>,
    state: ComputerState,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ComputerState {
    Runnable,
    Error,
    Finished,
}

impl Computer {
    pub fn new(memory: Vec<usize>) -> Self {
        Self {
            main_memory: memory,
            extended_memory: None,
            instruction_pointer: 0,
            instructions: HashMap::new(),
            state: ComputerState::Runnable,
        }
    }

    pub fn new_with_instructions(memory: Vec<usize>, instructions: Vec<Instruction>) -> Self {
        let mut c = Self::new(memory);
        for instruction in instructions {
            c.instructions.insert(instruction.get_code(), instruction);
        }
        c
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions
            .insert(instruction.get_code(), instruction);
    }

    pub fn set_memory(&mut self, memory: Vec<usize>) {
        self.main_memory = memory;
        let _ = self.extended_memory.take();
        self.instruction_pointer = 0;
        self.state = ComputerState::Runnable;
    }

    pub fn get(&self, address: usize) -> Option<usize> {
        if address > self.main_memory.len() - 1 {
            match &self.extended_memory {
                None => None,
                Some(ext) => {
                    if ext.contains_key(&address) {
                        Some(ext[&address])
                    } else {
                        None
                    }
                }
            }
        } else {
            Some(self.main_memory[address])
        }
    }

    pub fn set(&mut self, address: usize, value: usize) {
        if address > self.main_memory.len() - 1 {
            self.extended_memory
                .get_or_insert(Default::default())
                .insert(address, value);
        } else {
            self.main_memory[address] = value;
        }
    }

    pub fn set_status(&mut self, status: ComputerState) {
        self.state = status
    }

    pub fn get_status(&self) -> ComputerState {
        self.state
    }

    pub fn increase_ptr(&mut self, value: usize) {
        //TODO: Check validity before applying
        //eg. if larger than memory
        self.instruction_pointer += value;
    }

    pub fn step(&mut self) -> ComputerState {
        if self.state == ComputerState::Runnable {
            let intcode = match self.get(self.instruction_pointer) {
                Some(value) => value,
                None => {
                    self.state = ComputerState::Error;
                    return self.state;
                }
            };
            let instruction = self.instructions[&intcode].clone();
            instruction.run_instruction(self, self.instruction_pointer);
        }
        self.state
    }

    pub fn run_until_end(&mut self) -> ComputerState {
        while self.step() == Runnable {}
        self.state
    }
}
