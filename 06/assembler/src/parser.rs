use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;

use regex::Regex;
use crate::symbol_table::SymbolTable;

#[derive(Debug)]
pub enum AValue {
	Constant(u16),
	Variable(String)
}

#[derive(Debug)]
pub enum Target {
	M,
	D,
	MD,
	A,
	AM,
	AD,
	AMD
}

impl FromStr for Target {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> { 
		match s {
			"M" => Ok(Target::M),
			"D" => Ok(Target::D),
			"MD" => Ok(Target::MD),
			"A" => Ok(Target::A),
			"AM" => Ok(Target::AM),
			"AD" => Ok(Target::AD),
			"AMD" => Ok(Target::AMD),
			_ => {
				eprintln!("ERROR: Invalid token found {}", s);
				Err(())
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	Zero,
	One,
	MinusOne,
	D,
	A,
	NotD,
	NotA,
	MinusD,
	MinusA,
	IncD,
	IncA,
	DecD,
	DecA,
	DaddA,
	DminusA,
	AminusD,
	DandA,
	DorA,
	M,
	NotM,
	MinusM,
	IncM,
	DecM,
	DaddM,
	DminusM,
	MminusD,
	DandM,
	DorM
}

impl FromStr for OpCode {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"0" => Ok(OpCode::Zero),
			"1" => Ok(OpCode::One),
			"-1" => Ok(OpCode::MinusOne),
			"D" => Ok(OpCode::D),
			"A" => Ok(OpCode::A),
			"M" => Ok(OpCode::M),
			"!D" => Ok(OpCode::NotD),
			"!A" => Ok(OpCode::NotA),
			"!M" => Ok(OpCode::NotM),
			"-D" => Ok(OpCode::MinusD),
			"-A" => Ok(OpCode::MinusA),
			"-M" => Ok(OpCode::MinusM),
			"D+1" => Ok(OpCode::IncD),
			"A+1" => Ok(OpCode::IncA),
			"M+1" => Ok(OpCode::IncM),
			"D-1" => Ok(OpCode::DecD),
			"A-1" => Ok(OpCode::DecA),
			"M-1" => Ok(OpCode::DecM),
			"D+A" => Ok(OpCode::DaddA),
			"D+M" => Ok(OpCode::DaddM),
			"D-A" => Ok(OpCode::DminusA),
			"D-M" => Ok(OpCode::DminusM),
			"A-D" => Ok(OpCode::AminusD),
			"M-D" => Ok(OpCode::MminusD),
			"D&A" => Ok(OpCode::DandA),
			"D&M" => Ok(OpCode::DandM),
			"D|A" => Ok(OpCode::DorA),
			"D|M" => Ok(OpCode::DorM),
			_ => {
				eprintln!("ERROR: Invalid token found {}", s);
				Err(())
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum JumpSpec {
	JGT,
	JEQ,
	JGE,
	JLT,
	JNE,
	JLE,
	JMP
}

impl FromStr for JumpSpec {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"JGT" => Ok(JumpSpec::JGT),
			"JEQ" => Ok(JumpSpec::JEQ),
			"JGE" => Ok(JumpSpec::JGE),
			"JLT" => Ok(JumpSpec::JLT),
			"JNE" => Ok(JumpSpec::JNE),
			"JLE" => Ok(JumpSpec::JLE),
			"JMP" => Ok(JumpSpec::JMP),
			_ => {
				eprintln!("ERROR: Invalid token found {}", s);
				Err(())
			}
		}
	}
}

#[derive(Debug)]
pub enum Instruction {
	A(AValue),
	C(Option<Target>, Option<OpCode>, Option<JumpSpec>)
}

pub(crate) struct Parser<'a> {
	input_file: &'a str,
	instructions: Vec<Instruction>,
	symbol_table: SymbolTable
}

impl<'a> Parser<'a> {
	pub fn new(input_file: &'a str) -> Self {
		Parser { input_file: input_file, instructions: Vec::new(), symbol_table: SymbolTable::new() }
	}
	pub fn parse_file(&mut self) -> Result<(&Vec<Instruction>, &SymbolTable), io::Error> {
		let mut file_content = String::new();
		File::open(self.input_file)?.read_to_string(&mut file_content)?;
		
		let instructions: Vec<&str> = file_content.lines()
																							.map(|line| line.trim())
												  							 		  .filter(|line| {
												  											 line.len() > 0 && !line.starts_with("//")
																						  })
																							.collect();
		// figure out and register jump targets
		self.first_pass(&instructions);
		// check for A or C instructions
		self.second_pass(&instructions);
		// return result
		Ok((&self.instructions, &self.symbol_table))
	}
	
	fn first_pass(&mut self, instructions: &Vec<&str>) {
		let mut found_labels: usize = 0;
		for (i, code) in instructions.iter().enumerate() {
			if code.starts_with("(") {
					// JUMP target definition
					let value = code[1..code.len()-1].to_owned();
					if !self.symbol_table.has_symbol(&value) {
						let jump_target: u16 = u16::try_from(i - found_labels).unwrap();
					  self.symbol_table.register_symbol(&value, jump_target);
					  found_labels += 1;
					}				  
				}			
		}
	}

	fn second_pass(&mut self, instructions: &Vec<&str>) {
		let mut next_variable_location: u16 = 16;
		let re = Regex::new(r"((?P<target>\w{1,3})=)?((?P<opcode>[a-zA-Z0-9+\-!&\|]*))?(;(?P<jump>[a-zA-Z0-9]*))?").unwrap();
		
		for code in instructions {
			if code.starts_with("@") {
				// A instruction
				let value = code[1..].to_owned();
				match value.parse() {
				    Ok(constant_value) => { 
				    	self.instructions.push(Instruction::A(AValue::Constant(constant_value)));
				    }
				    Err(_) => {
				    	if !self.symbol_table.has_symbol(&value) {				    		
				    		self.symbol_table.register_symbol(&value, next_variable_location);
				    		next_variable_location += 1;				    		
				    	}
			    		self.instructions.push(Instruction::A(AValue::Variable(String::from(value)))); 
				    }
				}
			}
			else if code.starts_with("(") {
				// JUMP target definition
				// already handled in first pass
				continue;
			}
			else {
				// C instruction				
				let mut target: Option<Target> = None;
				let mut op_code: Option<OpCode> = None;
				let mut jump_spec: Option<JumpSpec> = None;
				
				let tokens = re.captures(code).unwrap();
				if let Some(tgt) = tokens.name("target") {
					if let Ok(t) = tgt.as_str().parse::<Target>() {
						target = Some(t);
					}
				}
				if let Some(opc) = tokens.name("opcode") {
					if let Ok(o) = opc.as_str().parse::<OpCode>() {
				  	op_code = Some(o);
					}
				}
				if let Some(jmp) = tokens.name("jump") {
					if let Ok(j) = jmp.as_str().parse::<JumpSpec>() {
				  	jump_spec = Some(j);
					}
				}				
				self.instructions.push(Instruction::C(target, op_code, jump_spec));
			}
		}
	}
}