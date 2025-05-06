mod asm_file;
mod register_allocator;

use crate::common::Register::Rax;
use crate::common::{Expr, Register, Type};
use crate::compiler::asm_file::AsmFile;
use crate::compiler::register_allocator::RegisterAllocator;
use crate::lexer::Token;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Compiler<'a> {
    reg_alloc: RegisterAllocator,
    asm_file: AsmFile,
    expression: &'a Expr<'a>,
    literals: HashMap<String, String>,
}

impl Compiler<'_> {
    pub fn new<'a>(expr: &'a Expr<'a>) -> Compiler<'a> {
        Compiler {
            expression: expr,
            reg_alloc: RegisterAllocator::new(),
            asm_file: AsmFile::new(),
            literals: HashMap::new(),
        }
    }

    fn get_mov_variation(t: &Type) -> &str {
        match t {
            Type::I32 => "mov",
            Type::F64 => "movsd",
        }
    }

    fn store_literal(&mut self, t: &Type, lit: &str) -> String {
        let typ = match t {
            Type::I32 => "dd",
            Type::F64 => "dq",
        };
        let key = format!("{}|{}", typ, lit);
        if let Some(literal) = self.literals.get(&key) {
            return literal.clone();
        }

        let literal = format!("__literal{}", self.literals.len());
        self.asm_file
            .write_rodata(&format!("{} {} {}", literal, typ, lit));
        self.literals.insert(key, literal.clone());
        literal
    }

    fn mov_l2r(&mut self, t: &Type, dst: &Register, literal: &str) {
        let instruction = Self::get_mov_variation(t);
        self.asm_file
            .write_instruction2(instruction, &dst.to_string(), literal);
    }

    fn mov_m2r(&mut self, t: &Type, dst: &Register, memory: &str) {
        let instruction = match t {
            Type::I32 => "mov",
            Type::F64 => "movsd",
        };
        self.asm_file
            .write_instruction2(instruction, &dst.to_string(), &format!("[{}]", memory));
    }

    fn mov_r2r(&mut self, t: &Type, dst: &Register, src: &Register) {
        let instruction = Self::get_mov_variation(t);
        self.asm_file
            .write_instruction2(instruction, &dst.to_string(), &src.to_string());
    }

    fn neg(&mut self, t: &Type, dst: &Register) {
        match t {
            Type::I32 => {
                self.asm_file.write_instruction1("neg", &dst.to_string());
            }
            Type::F64 => {
                let neg_literal = self.store_literal(t, "-1.0");
                let temp_temp = self.reg_alloc.alloc(t);
                self.mov_m2r(t, &temp_temp, &neg_literal);
                self.asm_file
                    .write_instruction2("mulsd", &dst.to_string(), &temp_temp.to_string());
                self.reg_alloc.dealloc(t);
            }
        };
    }

    fn add(&mut self, t: &Type, dst: &Register, src: &Register) {
        let instruction = match t {
            Type::I32 => "add",
            Type::F64 => "addsd",
        };
        self.asm_file
            .write_instruction2(instruction, &dst.to_string(), &src.to_string());
    }

    fn sub(&mut self, t: &Type, dst: &Register, src: &Register) {
        let instruction = match t {
            Type::I32 => "sub",
            Type::F64 => "subsd",
        };
        self.asm_file
            .write_instruction2(instruction, &dst.to_string(), &src.to_string());
    }

    fn imul(&mut self, t: &Type, dst: &Register, src: &Register) {
        let instruction = match t {
            Type::I32 => "imul",
            Type::F64 => "mulsd",
        };
        self.asm_file
            .write_instruction2(instruction, &dst.to_string(), &src.to_string());
    }

    fn div(&mut self, t: &Type, dst: &Register, src: &Register) {
        match t {
            Type::I32 => {
                self.mov_l2r(t, &Register::Rdx, "0");
                self.mov_r2r(t, &Rax, dst);
                self.asm_file.write_instruction1("idiv", &src.to_string());
                self.mov_r2r(t, dst, &Rax);
            }
            Type::F64 => {
                self.asm_file
                    .write_instruction2("divsd", &dst.to_string(), &src.to_string());
            }
        }
    }

    fn traverse_ast(&mut self, expr: &Expr) -> Result<(), Box<dyn Error>> {
        // 20 + ( 20 * 3 ) - ( 3 * 3 ) - 2
        // 1 + (2 + (3 + (4 + 5)))
        // 374 / 89 * 20 - 8901
        match expr {
            Expr::Binary(t, left, op, right) => {
                self.traverse_ast(left)?;
                self.traverse_ast(right)?;

                let src = self.reg_alloc.dealloc(t);
                let dst = self.reg_alloc.peek(t);

                match op {
                    Token::Plus => self.add(t, &dst, &src),
                    Token::Minus => self.sub(t, &dst, &src),
                    Token::Star => self.imul(t, &dst, &src),
                    Token::Slash => self.div(t, &dst, &src),
                    _ => return Err("Invalid operator".into()),
                };

                Ok(())
            }
            Expr::Grouping(_, group) => {
                self.traverse_ast(group)?;
                Ok(())
            }
            Expr::Unary(t, u) => {
                self.traverse_ast(u)?;
                let dst = self.reg_alloc.peek(t);
                self.neg(t, &dst);
                Ok(())
            }
            Expr::Literal(t, lit) => {
                let dst = self.reg_alloc.alloc(t);
                let literal = self.store_literal(t, lit);
                self.mov_m2r(t, &dst, &literal);
                Ok(())
            }
        }
    }

    pub fn compile(&mut self) -> Result<(), Box<dyn Error>> {
        self.traverse_ast(self.expression)?;
        fs::write("./target/program.asm", self.asm_file.finalize())?;
        Ok(())
    }
}
