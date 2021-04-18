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
		for (i, instruction) in self.instructions.iter().enumerate() {
			let mut assembly = String::new();
			assembly.push_str(&format!("//{}\n", instruction));
			match instruction {
				Instruction::Op(operation) => {
					match operation {
							// operations with two operands
					    Operation::Add |
					    Operation::Sub |
					    Operation::Eq  |
					    Operation::Gt  |
					    Operation::Lt  |
					    Operation::And |
					    Operation::Or => {
					    	assembly.push_str(&self.pop_from_stack());
					    	assembly.push_str(&self.select_mem_indirect("@SP"));
					    	assembly.push_str(&self.exec_operation(i, &operation));
					    	assembly.push_str(&self.push_to_stack());
					    },
					    // operations with one operand only
					    Operation::Neg | 
					    Operation::Not => {
					    	assembly.push_str(&self.select_mem_indirect("@SP"));
					    	assembly.push_str(&self.exec_operation(i, &operation));
					    	assembly.push_str(&self.push_to_stack());
					    }
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
					    	assembly.push_str(&self.load_const_value(*value));
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

	fn load_const_value(&self, value: u16) -> String {
		let mut assembly = String::new();
		assembly.push_str(&format!("@{}\n", value));
		assembly.push_str("D=A\n");
		assembly
	}

	fn push_to_stack(&self) -> String {
		let mut assembly = String::new();
		assembly.push_str(&self.select_mem_indirect("@SP"));
		assembly.push_str("M=D\n");
		assembly.push_str(&self.increase_pointer("@SP"));
		assembly
	}

	fn pop_from_stack(&self) -> String {
		let mut assembly = String::new();
		assembly.push_str(&self.decrease_pointer("@SP"));
		assembly.push_str("D=M\n");
		assembly
	}

	fn select_mem_indirect(&self, address: &str) -> String {
		let mut assembly = String::new();
		assembly.push_str(&format!("{}\n", address));
		assembly.push_str("A=M\n");
		assembly
	}

	fn exec_operation(&self, i: usize, op: &Operation) -> String {
		let mut assembly = String::new();
		assembly.push_str(&self.decrease_pointer("@SP"));
		match op {
		    Operation::Add => { assembly.push_str("D=D+M\n"); }
		    Operation::Sub => { assembly.push_str("D=M-D\n"); }
		    Operation::Eq => { 
		    	assembly.push_str("D=M-D\n"); 
		    	assembly.push_str(&format!("@EQ_TRUE_{}\n", i));
		    	assembly.push_str("D;JEQ\n");
		    	assembly.push_str("D=0\n");
		    	assembly.push_str(&format!("@EQ_END_{}\n", i));
		    	assembly.push_str("0;JMP\n");
		    	assembly.push_str(&format!("(EQ_TRUE_{})\n", i));
		    	assembly.push_str("D=-1\n");
		    	assembly.push_str(&format!("(EQ_END_{})\n", i));
		    }
		    Operation::Gt => { 
		    	assembly.push_str("D=M-D\n"); 
		    	assembly.push_str(&format!("@GT_TRUE_{}\n", i));
		    	assembly.push_str("D;JGT\n");
		    	assembly.push_str("D=0\n");
		    	assembly.push_str(&format!("@GT_END_{}\n", i));
		    	assembly.push_str("0;JMP\n");
		    	assembly.push_str(&format!("(GT_TRUE_{})\n", i));
		    	assembly.push_str("D=-1\n");
		    	assembly.push_str(&format!("(GT_END_{})\n", i));
		    }
		    Operation::Lt => { 
		    	assembly.push_str("D=M-D\n"); 
		    	assembly.push_str(&format!("@LT_TRUE_{}\n", i));
		    	assembly.push_str("D;JLT\n");
		    	assembly.push_str("D=0\n");
		    	assembly.push_str(&format!("@LT_END_{}\n", i));
		    	assembly.push_str("0;JMP\n");
		    	assembly.push_str(&format!("(LT_TRUE_{})\n", i));
		    	assembly.push_str("D=-1\n");
		    	assembly.push_str(&format!("(LT_END_{})\n", i));
		    }
		    Operation::And => { assembly.push_str("D=D&M\n"); }
		    Operation::Or => { assembly.push_str("D=D|M\n"); }
		    Operation::Neg => { assembly.push_str("D=-M\n"); }
		    Operation::Not => { assembly.push_str("D=!M\n"); }
		}
		assembly
	}

	fn increase_pointer(&self, address: &str) -> String {
		let mut assembly = String::new();
		assembly.push_str(&format!("{}\n", address));
		assembly.push_str("M=M+1\n");
		assembly
	}

	fn decrease_pointer(&self, address: &str) -> String {
		let mut assembly = String::new();
		assembly.push_str(&format!("{}\n", address));
		assembly.push_str("M=M-1\n");
		assembly.push_str("A=M\n");
		assembly
	}
}