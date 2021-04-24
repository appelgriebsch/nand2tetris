use std::{fs::File, path::PathBuf};
use std::io;
use std::io::Write;

use crate::parser::{Instruction, Operation, Segment};

#[derive(Debug)]
pub(crate) struct CodeGen<'a> {
  output_file: &'a str,
  file: File
}

impl <'a> CodeGen<'a> {
  pub fn new(output_file: &'a str, ) -> Self {
    let file = File::create(output_file).unwrap();
    CodeGen { output_file: output_file, file: file }
  }
  pub fn init(&self) -> Result<(), io::Error> {
    Ok(())
  }
  pub fn generate(&mut self, instructions: &Vec<Instruction>, debug: bool) -> Result<(), io::Error> {
    let file_path = PathBuf::from(self.output_file);
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    for (i, instruction) in instructions.iter().enumerate() {
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
              Segment::Local => {
                assembly.push_str(&self.select_segment_index("@LCL", *value));
                assembly.push_str(&self.pop_from_stack());
                assembly.push_str(&self.push_to_segment("@LCL"));
                assembly.push_str(&self.deselect_segment_index("@LCL", *value));
              }
              Segment::Argument => {
                assembly.push_str(&self.select_segment_index("@ARG", *value));
                assembly.push_str(&self.pop_from_stack());
                assembly.push_str(&self.push_to_segment("@ARG"));
                assembly.push_str(&self.deselect_segment_index("@ARG", *value));
              }
              Segment::This => {
                assembly.push_str(&self.select_segment_index("@THIS", *value));
                assembly.push_str(&self.pop_from_stack());
                assembly.push_str(&self.push_to_segment("@THIS"));
                assembly.push_str(&self.deselect_segment_index("@THIS", *value));
              }
              Segment::That => {
                assembly.push_str(&self.select_segment_index("@THAT", *value));
                assembly.push_str(&self.pop_from_stack());
                assembly.push_str(&self.push_to_segment("@THAT"));
                assembly.push_str(&self.deselect_segment_index("@THAT", *value));
              }
              Segment::Temp => {
                assembly.push_str(&self.pop_from_stack());
                assembly.push_str(&format!("@R{}\n", (5 + *value)));
                assembly.push_str("M=D\n");
              }
              Segment::Pointer => {
                if *value == 0 {
                  assembly.push_str(&self.pop_from_stack());
                  assembly.push_str("@THIS\n");
                  assembly.push_str("M=D\n");
                }
                else {
                  assembly.push_str(&self.pop_from_stack());
                  assembly.push_str("@THAT\n");
                  assembly.push_str("M=D\n");
                }
              }
              Segment::Static => {
                assembly.push_str(&self.pop_from_stack());
                assembly.push_str(&format!("@{}.{}\n", file_name, *value));
                assembly.push_str("M=D\n");
              }
          }
        },
        Instruction::Push(segment, value) => {
          match segment {
              Segment::Constant => {
                assembly.push_str(&self.load_const_value(*value));
                assembly.push_str(&self.push_to_stack());
              }
              Segment::Local => {
                assembly.push_str(&self.select_segment_index("@LCL", *value));
                assembly.push_str("A=M\n");
                assembly.push_str("D=M\n");
                assembly.push_str(&self.push_to_stack());
                assembly.push_str(&self.deselect_segment_index("@LCL", *value));
              }
              Segment::Argument => {
                assembly.push_str(&self.select_segment_index("@ARG", *value));
                assembly.push_str("A=M\n");
                assembly.push_str("D=M\n");
                assembly.push_str(&self.push_to_stack());
                assembly.push_str(&self.deselect_segment_index("@ARG", *value));
              }
              Segment::This => {
                assembly.push_str(&self.select_segment_index("@THIS", *value));
                assembly.push_str("A=M\n");
                assembly.push_str("D=M\n");
                assembly.push_str(&self.push_to_stack());
                assembly.push_str(&self.deselect_segment_index("@THIS", *value));
              }
              Segment::That => {
                assembly.push_str(&self.select_segment_index("@THAT", *value));
                assembly.push_str("A=M\n");
                assembly.push_str("D=M\n");
                assembly.push_str(&self.push_to_stack());
                assembly.push_str(&self.deselect_segment_index("@THAT", *value));
              }
              Segment::Temp => {
                assembly.push_str(&format!("@R{}\n", (5 + *value)));
                assembly.push_str("D=M\n");
                assembly.push_str(&self.push_to_stack());
              }
              Segment::Pointer => {
                if *value == 0 {
                  assembly.push_str("@THIS\n");
                  assembly.push_str("D=M\n");
                  assembly.push_str(&self.push_to_stack());
                }
                else {
                  assembly.push_str("@THAT\n");
                  assembly.push_str("D=M\n");
                  assembly.push_str(&self.push_to_stack());
                }
              }
              Segment::Static => {
                assembly.push_str(&format!("@{}.{}\n", file_name, *value));
                assembly.push_str("D=M\n");
                assembly.push_str(&self.push_to_stack());
              }
          }
        },
        Instruction::Label(_) => {}
        Instruction::Goto(_) => {}
        Instruction::IfGoto(_) => {}
        Instruction::DefFunc(_, _) => {}
        Instruction::CallFunc(_, _) => {}
        Instruction::Return => {}
      }
      if debug {
        println!("{:?} -> {:?}", instruction, assembly);
      }
      self.file.write_fmt(format_args!("{}\n", assembly))?;
    }
    self.file.flush()
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

  fn select_segment_index(&self, base: &str, idx: u16) -> String {
    let mut assembly = String::new();
    assembly.push_str(&format!("@{}\n", idx));
    assembly.push_str("D=A\n");
    assembly.push_str(&format!("{}\n", base));
    assembly.push_str("M=M+D\n");
    assembly
  }

  fn deselect_segment_index(&self, base: &str, idx: u16) -> String {
    let mut assembly = String::new();
    assembly.push_str(&format!("@{}\n", idx));
    assembly.push_str("D=A\n");
    assembly.push_str(&format!("{}\n", base));
    assembly.push_str("M=M-D\n");
    assembly
  }

  fn push_to_segment(&self, base: &str) -> String {
    let mut assembly = String::new();
    assembly.push_str(&format!("{}\n", base));
    assembly.push_str("A=M\n");
    assembly.push_str("M=D\n");
    assembly
  }

  fn exec_operation(&self, i: usize, op: &Operation) -> String {
    let mut assembly = String::new();
    assembly.push_str(&self.decrease_pointer("@SP"));
    assembly.push_str("A=M\n");
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
    assembly
  }
}