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

#[derive(Debug, Clone)]
pub enum Instruction {
    Op(Operation),
    Pop(Segment, u16),
    Push(Segment, u16),
    Label(String),
    Goto(String),
    IfGoto(String),
    DefFunc(String, u16),
    CallFunc(String, u16),
    Return
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
            Instruction::Label(name) => { f.write_fmt(format_args!("label {}", name)) }
            Instruction::Goto(label) => { f.write_fmt(format_args!("goto {}", label)) }
            Instruction::IfGoto(label) => { f.write_fmt(format_args!("if-goto {}", label)) }
            Instruction::DefFunc(name, no_of_locals) => { f.write_fmt(format_args!("function {} ({})", name, no_of_locals)) }
            Instruction::CallFunc(name, no_of_args) => { f.write_fmt(format_args!("call {} ({})", name, no_of_args)) }
            Instruction::Return => { f.write_str("return") }
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
        
        let instructions: Vec<String> = file_content.lines()
                                                  .map(|line| line.trim())
                                                  .map(|line| {
                                                    if let Some(comment) = line.find("//") {
                                                        line[0..comment].to_owned()
                                                    }
                                                    else {
                                                        line.to_owned()
                                                    }
                                                  })
                                                  .filter(|line| {
                                                      line.len() > 0 && !line.starts_with("//")
                                                  })
                                                  .map(|line| line.trim().to_owned())
                                                  .collect();
        
        for code in instructions {
            let parts: Vec<&str> = code.split(' ').collect();
            if parts.is_empty() {
                continue;
            }
            if let Ok(operation) = parts[0].trim().parse::<Operation>() {
                self.instructions.push(Instruction::Op(operation));
            }
            else {
                match parts[0].trim() {
                    "push" => {
                        if let Ok(segment) = parts[1].trim().parse::<Segment>() {
                            self.instructions.push(Instruction::Push(segment, 
                                 parts[2].trim().parse().expect(&format!("Unable to parse {}", parts[2]))));
                        }
                    },
                    "pop" => {
                        if let Ok(segment) = parts[1].trim().parse::<Segment>() {
                            self.instructions.push(Instruction::Pop(segment, 
                                 parts[2].trim().parse().expect(&format!("Unable to parse {}", parts[2]))));
                        }
                    },
                    "goto" => {
                        self.instructions.push(Instruction::Goto(parts[1].trim().to_owned()));
                    },
                    "label" => {
                        self.instructions.push(Instruction::Label(parts[1].trim().to_owned()));
                    }
                    "if-goto" => {
                        self.instructions.push(Instruction::IfGoto(parts[1].trim().to_owned()));
                    },
                    "function" => {
                        self.instructions.push(Instruction::DefFunc(parts[1].trim().to_owned(), 
                                 parts[2].trim().parse().expect(&format!("Unable to parse {}", parts[2]))));
                    },
                    "call" => {
                        self.instructions.push(Instruction::CallFunc(parts[1].trim().to_owned(),
                                 parts[2].trim().parse().expect(&format!("Unable to parse {}", parts[2]))));
                    },
                    "return" => {
                        self.instructions.push(Instruction::Return);
                    },
                    _ => {
                        eprintln!("ERROR: Invalid token found {} in line {}", parts[0], parts.join(" "));               
                    }
                }
            }
        }
        Ok(&self.instructions)
    }
}
