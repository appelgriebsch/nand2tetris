use std::fs::File;
use std::io;
use std::io::Write;

use crate::parser::{Instruction, Operation, Segment};

#[derive(Debug)]
pub(crate) struct CodeGen<'a> {
	output_file: &'a str,
	instructions: &'a Vec<Instruction>
}

impl <'a> CodeGen<'a> {
	pub fn new(output_file: &'a str, instructions: &'a Vec<Instruction>) -> Self {
		CodeGen { output_file: output_file, instructions: instructions }
	}
	pub fn generate(&self, debug: bool) -> Result<(), io::Error> {
		let mut file = File::create(self.output_file)?;
		for instruction in self.instructions {
			let mut assembly = String::new();
			assembly.push_str(&format!("//{}\n", instruction));
			match instruction {
				Instruction::Op(operation) => {
					match operation {
					    Operation::Add => {
					    	assembly.push_str(&self.pop_from_stack());
					    	assembly.push_str(&self.select_mem_indirect("@SP"));
					    	assembly.push_str(&self.op_add());
					    	assembly.push_str(&self.push_to_stack());
					    }
					    Operation::Sub => {}
					    Operation::Neg => {}
					    Operation::Eq => {}
					    Operation::Gt => {}
					    Operation::Lt => {}
					    Operation::And => {}
					    Operation::Or => {}
					    Operation::Not => {}
					}
				},
				Instruction::Pop(segment, value) => {
					match segment {
					    Segment::Constant => {					    	
					    	continue; // no need to pop const values from anywhere
					    }
					    Segment::Local => {}
					    Segment::Argument => {}
					    Segment::This => {}
					    Segment::That => {}
					    Segment::Temp => {}
					    Segment::Pointer => {}
					    Segment::Static => {}
					}
				},
				Instruction::Push(segment, value) => {
					match segment {
					    Segment::Constant => {
					    	assembly.push_str(&self.load_value(*value));
					    	assembly.push_str(&self.push_to_stack());
					    }
					    Segment::Local => {}
					    Segment::Argument => {}
					    Segment::This => {}
					    Segment::That => {}
					    Segment::Temp => {}
					    Segment::Pointer => {}
					    Segment::Static => {}
					}
				}
			}
			if debug {
				println!("{:?} -> {:?}", instruction, assembly);
			}
			file.write_fmt(format_args!("{}\n", assembly))?;
		}
		file.flush()
	}

	fn load_value(&self, value: u16) -> String {
		let mut assembly = String::new();
		assembly.push_str(&format!("@{}\n", value));
		assembly.push_str("D=A\n");
		assembly
	}

	fn push_to_stack(&self) -> String {
		let mut assembly = String::new();
		assembly.push_str("@SP\n");
		assembly.push_str("A=M\n");
		assembly.push_str("M=D\n");
		assembly.push_str("@SP\n");
		assembly.push_str("M=M+1\n");
		assembly
	}

	fn pop_from_stack(&self) -> String {
		let mut assembly = String::new();
		assembly.push_str("@SP\n");
		assembly.push_str("M=M-1\n");
		assembly.push_str("A=M\n");
		assembly.push_str("D=M\n");
		assembly
	}

	fn select_mem_indirect(&self, address: &str) -> String {
		let mut assembly = String::new();
		assembly.push_str(&format!("{}\n", address));
		assembly.push_str("A=M\n");
		assembly
	}

	fn op_add(&self) -> String {
		let mut assembly = String::new();
		assembly.push_str("@SP\n");
		assembly.push_str("M=M-1\n");
		assembly.push_str("A=M\n");
		assembly.push_str("D=D+M\n");
		assembly.push_str(&self.select_mem_indirect("@SP"));
		assembly.push_str("M=M-1\n");
		assembly
	}
}