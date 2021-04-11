use std::fs::File;
use std::io;
use std::io::Write;

use crate::{parser::{AValue,Instruction,Target,OpCode,JumpSpec}, symbol_table::SymbolTable};

impl ToString for Target {
	fn to_string(&self) -> String { 
		match self {
			Target::M => "001".to_string(),
			Target::D => "010".to_string(),
			Target::MD => "011".to_string(),
			Target::A => "100".to_string(),
			Target::AM => "101".to_string(),
			Target::AD => "110".to_string(),
			Target::AMD => "111".to_string()
		}
	}
}

impl ToString for OpCode {
	fn to_string(&self) -> String {
		match self {
		    OpCode::Zero => "0101010".to_string(),
		    OpCode::One => "0111111".to_string(),
		    OpCode::MinusOne => "0111010".to_string(),
		    OpCode::D => "0001100".to_string(),
		    OpCode::A => "0110000".to_string(),
		    OpCode::NotD => "0001101".to_string(),
		    OpCode::NotA => "0110001".to_string(),
		    OpCode::MinusD => "0001111".to_string(),
		    OpCode::MinusA => "0110011".to_string(),
		    OpCode::IncD => "0011111".to_string(),
		    OpCode::IncA => "0110111".to_string(),
		    OpCode::DecD => "0001110".to_string(),
		    OpCode::DecA => "0110010".to_string(),
		    OpCode::DaddA => "0000010".to_string(),		    
		    OpCode::DminusA => "0010011".to_string(),
		    OpCode::AminusD => "0000111".to_string(),
		    OpCode::DandA => "0000000".to_string(),
		    OpCode::DorA => "0010101".to_string(),
		    OpCode::M => "1110000".to_string(),
		    OpCode::NotM => "1110001".to_string(),
		    OpCode::MinusM => "1110011".to_string(),
		    OpCode::IncM => "1110111".to_string(),
		    OpCode::DecM => "1110010".to_string(),
		    OpCode::DaddM => "1000010".to_string(),
		    OpCode::DminusM => "1010011".to_string(),
		    OpCode::MminusD => "1000111".to_string(),
		    OpCode::DandM => "1000000".to_string(),
		    OpCode::DorM => "1010101".to_string()
		}
	}
}

impl ToString for JumpSpec {
	fn to_string(&self) -> String {
		match self {
			JumpSpec::JGT => "001".to_string(),
			JumpSpec::JEQ => "010".to_string(),
			JumpSpec::JGE => "011".to_string(),
			JumpSpec::JLT => "100".to_string(),
			JumpSpec::JNE => "101".to_string(),
			JumpSpec::JLE => "110".to_string(),
			JumpSpec::JMP => "111".to_string()			
		}
	}
}

pub(crate) struct CodeGen<'a> {
	output_file: &'a str,
	instructions: &'a Vec<Instruction>,
	symbol_table: &'a SymbolTable
}

impl <'a> CodeGen<'a> {
	pub fn new(output_file: &'a str, instructions: &'a Vec<Instruction>, symbol_table: &'a SymbolTable) -> Self {
		CodeGen { output_file: output_file, instructions: instructions, symbol_table: symbol_table }
	}
	pub fn generate(&self, debug: bool) -> Result<(), io::Error> {
		let mut file = File::create(self.output_file)?;		
		for instruction in self.instructions {
			let mut binary_code = String::new();
			match instruction {
			    Instruction::A(val) => {
			    	binary_code.push('0');
			    	let value = match val {
			    		AValue::Constant(c) => c,
			    		AValue::Variable(symbol) => {
			    			self.symbol_table.get_symbol(&symbol).unwrap()
			    		}
			    	};
			    	binary_code.push_str(&format!("{:015b}", value));
			    },
			    Instruction::C(target, opcode, jump_target) => {
			    	binary_code.push_str("111");
			    	let op = match opcode {
			    	    Some(o) => o.to_string(),
			    	    None => format!("{:07b}", 0)
			    	};
			    	let dest = match target {
			    	    Some(t) => t.to_string(),
			    	    None => format!("{:03b}", 0)
			    	};
			    	let jump = match jump_target {
			    	    Some(j) => j.to_string(),
			    	    None => format!("{:03b}", 0)
			    	};
			    	binary_code.push_str(&format!("{}{}{}", op, dest, jump));
			    }
			}
			if debug {
				println!("{:?} -> {:?}", instruction, binary_code);
			}

			file.write_fmt(format_args!("{}\n", binary_code))?;
		}
		file.flush()		
	}
}