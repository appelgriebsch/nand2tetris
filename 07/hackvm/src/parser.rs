use std::{fs::File, str::FromStr};
use std::io;
use std::io::Read;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Segment {
	Constant,
	Local,
	Argument,
	This,
	That,
	Temp,
	Pointer,
	Static
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
	Add,
	Sub,
	Neg,
	Eq,
	Gt,
	Lt,
	And,
	Or,
	Not
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	Op(Operation),
	Pop(Segment, u16),
	Push(Segment, u16),
//Label(String),
//Goto(String),
//Cond(String),
//Function,
//Call,
//Return
}

impl FromStr for Segment {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
    	match s {
    		"constant" => Ok(Segment::Constant),
    		"local" => Ok(Segment::Local),
    		"argument" => Ok(Segment::Argument),
    		"this" => Ok(Segment::This),
    		"that" => Ok(Segment::That),
    		"temp" => Ok(Segment::Temp),
    		"pointer" => Ok(Segment::Pointer),
    		"static" => Ok(Segment::Static),
    		_ => {
        		eprintln!("ERROR: Invalid token found {}", s);
        		Err(())
        }
    	}
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self {
    	    Segment::Constant => { f.write_str("constant") }
    	    Segment::Local => { f.write_str("local") }
    	    Segment::Argument => { f.write_str("argument") }
    	    Segment::This => { f.write_str("this") }
    	    Segment::That => { f.write_str("that") }
    	    Segment::Temp => { f.write_str("temp") }
    	    Segment::Pointer => { f.write_str("pointer") }
    	    Segment::Static => { f.write_str("static") }
    	}
    }
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
        	"add" => Ok(Operation::Add),
        	"sub" => Ok(Operation::Sub),
        	"neg" => Ok(Operation::Neg),
        	"eq" => Ok(Operation::Eq),
        	"gt" => Ok(Operation::Gt),
        	"lt" => Ok(Operation::Lt),
        	"and" => Ok(Operation::And),
        	"or" => Ok(Operation::Or),
        	"not" => Ok(Operation::Not),
        	_ => {
        		eprintln!("ERROR: Invalid token found {}", s);
        		Err(())
        	}
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self {
    	    Operation::Add => { f.write_str("add") }
    	    Operation::Sub => { f.write_str("sub") }
    	    Operation::Neg => { f.write_str("neg") }
    	    Operation::Eq => { f.write_str("eq") }
    	    Operation::Gt => { f.write_str("gt") }
    	    Operation::Lt => { f.write_str("lt") }
    	    Operation::And => { f.write_str("and") }
    	    Operation::Or => { f.write_str("or") }
    	    Operation::Not => { f.write_str("not") }
    	}
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Op(operation) => { f.write_fmt(format_args!("{}", operation)) }
            Instruction::Pop(segment, value) => { f.write_fmt(format_args!("pop {} {}", segment, value)) }
            Instruction::Push(segment, value) => { f.write_fmt(format_args!("push {} {}", segment, value)) }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Parser<'a> {
	input_file: &'a str,
	instructions: Vec<Instruction>
}

impl<'a> Parser<'a> {
	pub fn new(input_file: &'a str) -> Self {
		Parser { input_file: input_file, instructions: Vec::new() }
	}
	pub fn parse_file(&mut self) -> Result<&Vec<Instruction>, io::Error> {
		let mut file_content = String::new();
		File::open(self.input_file)?.read_to_string(&mut file_content)?;
		
		let instructions: Vec<&str> = file_content.lines()
																							.map(|line| line.trim())
												  							 		  .filter(|line| {
												  											 line.len() > 0 && !line.starts_with("//")
																						  })
																							.collect();
		
		for code in instructions {
			let parts: Vec<&str> = code.split(' ').collect();
			if parts.is_empty() {
				continue;
			}
			else if parts.len() == 1 {
				if let Ok(operation) = parts[0].parse::<Operation>() {
					self.instructions.push(Instruction::Op(operation));
				}
			}
			else {
				match parts[0] {
					"push" => {
						if let Ok(segment) = parts[1].parse::<Segment>() {
							self.instructions.push(Instruction::Push(segment, parts[2].parse().unwrap()));
						}
					},
					"pop" => {
						if let Ok(segment) = parts[1].parse::<Segment>() {
							self.instructions.push(Instruction::Pop(segment, parts[2].parse().unwrap()));
						}
					},
					_ => {
        		eprintln!("ERROR: Invalid token found {}", parts[0]);        		
        	}
				}
			}
		}
		Ok(&self.instructions)
	}
}
