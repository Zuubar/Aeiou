use crate::common::Register::{Rcx, Rdi, Rsi, Xmm0, Xmm1, Xmm2};
use crate::common::{Register, Type};

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

    pub fn current(&mut self) -> Register {
        self.stack[self.stack_ptr - 1].clone()
    }

    pub fn next(&mut self) -> Register {
        self.stack[self.stack_ptr].clone()
    }

    pub fn dealloc(&mut self) -> Register {
        self.stack_ptr -= 1;
        self.current()
    }

    pub fn alloc(&mut self) -> Register {
        self.stack_ptr += 1;
        self.current()
    }
}

pub struct RegisterAllocator {
    stack: RegisterStack,
    stack_simd: RegisterStack,
}

impl RegisterAllocator {
    pub fn new() -> RegisterAllocator {
        RegisterAllocator {
            stack: RegisterStack::new(vec![Rdi, Rsi, Rcx]),
            stack_simd: RegisterStack::new(vec![Xmm0, Xmm1, Xmm2]),
        }
    }

    fn get_stack(&mut self, t: &Type) -> &mut RegisterStack {
        match t {
            Type::I32 => &mut self.stack,
            Type::F64 => &mut self.stack_simd,
        }
    }

    pub fn current(&mut self, t: &Type) -> Register {
        self.get_stack(t).current()
    }

    pub fn next(&mut self, t: &Type) -> Register {
        self.get_stack(t).next()
    }

    pub fn dealloc(&mut self, t: &Type) -> Register {
        self.get_stack(t).dealloc()
    }

    pub fn alloc(&mut self, t: &Type) -> Register {
        self.get_stack(t).alloc()
    }
}
