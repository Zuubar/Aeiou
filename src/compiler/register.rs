use crate::compiler::register::Register::{Rcx, Rdi, Rsi, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7, R10, R11, R8, R9};
use crate::parser::Type;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Register {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Xmm0,
    Xmm1,
    Xmm2,
    Xmm3,
    Xmm4,
    Xmm5,
    Xmm6,
    Xmm7,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lowered = format!("{:?}", self).to_lowercase();
        write!(f, "{}", lowered)
    }
}

struct RegisterStack {
    stack: Vec<Register>,
    stack_ptr: usize,
}

impl RegisterStack {
    pub fn new(registers: Vec<Register>) -> RegisterStack {
        RegisterStack {
            stack: registers,
            stack_ptr: 0,
        }
    }

    pub fn peek(&mut self) -> Register {
        self.stack[self.stack_ptr - 1].clone()
    }

    pub fn dealloc(&mut self) -> Register {
        self.stack_ptr -= 1;
        self.stack[self.stack_ptr].clone()
    }

    pub fn alloc(&mut self) -> Register {
        self.stack_ptr += 1;
        self.stack[self.stack_ptr - 1].clone()
    }
}

pub struct RegisterAllocator {
    stack: RegisterStack,
    stack_simd: RegisterStack,
}

impl RegisterAllocator {
    pub fn new() -> RegisterAllocator {
        RegisterAllocator {
            stack: RegisterStack::new(vec![Rcx, Rdi, Rsi, R8, R9, R10, R11]),
            stack_simd: RegisterStack::new(vec![Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7]),
        }
    }

    fn get_stack(&mut self, t: &Type) -> &mut RegisterStack {
        match t {
            Type::I32 => &mut self.stack,
            Type::F64 => &mut self.stack_simd,
        }
    }

    pub fn peek(&mut self, t: &Type) -> Register {
        self.get_stack(t).peek()
    }

    pub fn dealloc(&mut self, t: &Type) -> Register {
        self.get_stack(t).dealloc()
    }

    pub fn alloc(&mut self, t: &Type) -> Register {
        let s = self.get_stack(t);
        if s.stack_ptr >= s.stack.len() {
            panic!("Out of registers!")
        }
        self.get_stack(t).alloc()
    }
}
