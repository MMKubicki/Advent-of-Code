use crate::intcode::{Code, Computer};
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Instruction {
    code: Code,
    action: Rc<dyn Fn(&mut Computer, usize)>,
}

impl Instruction {
    pub fn new<I: 'static + Fn(&mut Computer, usize)>(code: Code, action: I) -> Instruction {
        Self {
            code,
            action: Rc::new(action),
        }
    }

    pub fn get_code(&self) -> Code {
        self.code
    }

    pub fn run_instruction(&self, computer: &mut Computer, address: usize) {
        (self.action)(computer, address)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction {{code: {}, action: ... }}", self.code)
    }
}

pub fn default_instruction_appy<F: Fn(usize, usize) -> usize>(
    computer: &mut Computer,
    address: usize,
    todo: F,
) {
    let addr1 = computer.get(address + 1).unwrap_or_else(|| {
        panic!(
            "Failed to get value at {} for first address of operation at {}",
            address + 1,
            address
        )
    });
    let addr2 = computer.get(address + 2).unwrap_or_else(|| {
        panic!(
            "Failed to get value at {} for second address of operation at {}",
            address + 2,
            address
        )
    });
    let addr_res = computer.get(address + 3).unwrap_or_else(|| {
        panic!(
            "Failed to get value at {} for result address of operation at {}",
            address + 3,
            address
        )
    });

    let value1 = computer.get(addr1).unwrap_or_else(|| {
        panic!(
            "Failed to get value at {} for first value of operation at {}",
            addr1, address
        )
    });
    let value2 = computer.get(addr2).unwrap_or_else(|| {
        panic!(
            "Failed to get value at {} for first value of operation at {}",
            addr2, address
        )
    });

    computer.set(addr_res, todo(value1, value2));
    computer.increase_ptr(4);
}
