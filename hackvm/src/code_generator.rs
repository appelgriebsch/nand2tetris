use std::io;
use std::io::Write;
use std::fs::File;
use crate::parser::{Instruction, Operation, Segment};

#[derive(Debug)]
pub(crate) struct CodeGen<'a> {
    output_file: &'a str,
    file: File,
}

impl<'a> CodeGen<'a> {
    pub fn new(output_file: &'a str) -> Self {
        let file = File::create(output_file).unwrap();
        CodeGen {
            output_file: output_file,
            file: file,
        }
    }
    pub fn init(&self) -> Result<(), io::Error> {
        Ok(())
    }
    pub fn generate(
        &mut self,
        file_name: &str,
        instructions: &Vec<Instruction>,
        debug: bool,
    ) -> Result<(), io::Error> {
        let mut current_function: String = String::new();
        let mut current_fn_return: u16 = 0;
        for (i, instruction) in instructions.iter().enumerate() {
            let mut assembly = String::new();
            assembly.push_str(&format!("//{}\n", instruction));
            match instruction {
                Instruction::Op(operation) => {
                    match operation {
                        // operations with two operands
                        Operation::Add
                        | Operation::Sub
                        | Operation::Eq
                        | Operation::Gt
                        | Operation::Lt
                        | Operation::And
                        | Operation::Or => {
                            self.pop_from_stack(&mut assembly);
                            self.select_mem_indirect("@SP", &mut assembly);
                            self.exec_operation(i, &operation, &mut assembly);
                            self.push_to_stack(&mut assembly);
                        }
                        // operations with one operand only
                        Operation::Neg | Operation::Not => {
                            self.select_mem_indirect("@SP", &mut assembly);
                            self.exec_operation(i, &operation, &mut assembly);
                            self.push_to_stack(&mut assembly);
                        }
                    }
                }
                Instruction::Pop(segment, value) => {
                    match segment {
                        Segment::Constant => {
                            continue; // no need to pop const values from anywhere
                        }
                        Segment::Local => {
                            self.select_segment_index("@LCL", *value, &mut assembly);
                            self.pop_from_stack(&mut assembly);
                            self.push_to_segment("@LCL", &mut assembly);
                            self.deselect_segment_index("@LCL", *value, &mut assembly);
                        }
                        Segment::Argument => {
                            self.select_segment_index("@ARG", *value, &mut assembly);
                            self.pop_from_stack(&mut assembly);
                            self.push_to_segment("@ARG", &mut assembly);
                            self.deselect_segment_index("@ARG", *value, &mut assembly);
                        }
                        Segment::This => {
                            self.select_segment_index("@THIS", *value, &mut assembly);
                            self.pop_from_stack(&mut assembly);
                            self.push_to_segment("@THIS", &mut assembly);
                            self.deselect_segment_index("@THIS", *value, &mut assembly);
                        }
                        Segment::That => {
                            self.select_segment_index("@THAT", *value, &mut assembly);
                            self.pop_from_stack(&mut assembly);
                            self.push_to_segment("@THAT", &mut assembly);
                            self.deselect_segment_index("@THAT", *value, &mut assembly);
                        }
                        Segment::Temp => {
                            self.pop_from_stack(&mut assembly);
                            assembly.push_str(&format!("@R{}\n", (5 + *value)));
                            assembly.push_str("M=D\n");
                        }
                        Segment::Pointer => {
                            if *value == 0 {
                                self.pop_from_stack(&mut assembly);
                                assembly.push_str("@THIS\n");
                                assembly.push_str("M=D\n");
                            } else {
                                self.pop_from_stack(&mut assembly);
                                assembly.push_str("@THAT\n");
                                assembly.push_str("M=D\n");
                            }
                        }
                        Segment::Static => {
                            self.pop_from_stack(&mut assembly);
                            assembly.push_str(&format!("@{}.{}\n", file_name, *value));
                            assembly.push_str("M=D\n");
                        }
                    }
                }
                Instruction::Push(segment, value) => match segment {
                    Segment::Constant => {
                        self.load_const_value(*value, &mut assembly);
                        self.push_to_stack(&mut assembly);
                    }
                    Segment::Local => {
                        self.select_segment_index("@LCL", *value, &mut assembly);
                        assembly.push_str("A=M\n");
                        assembly.push_str("D=M\n");
                        self.push_to_stack(&mut assembly);
                        self.deselect_segment_index("@LCL", *value, &mut assembly);
                    }
                    Segment::Argument => {
                        self.select_segment_index("@ARG", *value, &mut assembly);
                        assembly.push_str("A=M\n");
                        assembly.push_str("D=M\n");
                        self.push_to_stack(&mut assembly);
                        self.deselect_segment_index("@ARG", *value, &mut assembly);
                    }
                    Segment::This => {
                        self.select_segment_index("@THIS", *value, &mut assembly);
                        assembly.push_str("A=M\n");
                        assembly.push_str("D=M\n");
                        self.push_to_stack(&mut assembly);
                        self.deselect_segment_index("@THIS", *value, &mut assembly);
                    }
                    Segment::That => {
                        self.select_segment_index("@THAT", *value, &mut assembly);
                        assembly.push_str("A=M\n");
                        assembly.push_str("D=M\n");
                        self.push_to_stack(&mut assembly);
                        self.deselect_segment_index("@THAT", *value, &mut assembly);
                    }
                    Segment::Temp => {
                        assembly.push_str(&format!("@R{}\n", (5 + *value)));
                        assembly.push_str("D=M\n");
                        self.push_to_stack(&mut assembly);
                    }
                    Segment::Pointer => {
                        if *value == 0 {
                            assembly.push_str("@THIS\n");
                            assembly.push_str("D=M\n");
                            self.push_to_stack(&mut assembly);
                        } else {
                            assembly.push_str("@THAT\n");
                            assembly.push_str("D=M\n");
                            self.push_to_stack(&mut assembly);
                        }
                    }
                    Segment::Static => {
                        assembly.push_str(&format!("@{}.{}\n", file_name, *value));
                        assembly.push_str("D=M\n");
                        self.push_to_stack(&mut assembly);
                    }
                },
                Instruction::Label(name) => {
                    assembly.push_str(&format!("({}.{}${})\n", file_name, current_function, name));
                }
                Instruction::Goto(label) => {
                    assembly.push_str(&format!("@{}.{}${}\n", file_name, current_function, label));
                    assembly.push_str("0;JMP\n");
                }
                Instruction::IfGoto(label) => {
                    self.pop_from_stack(&mut assembly);
                    assembly.push_str(&format!("@{}.{}${}\n", file_name, current_function, label));
                    assembly.push_str("D;JNE\n");
                }
                Instruction::DefFunc(name, no_of_locals) => {
                    current_function = name.clone();
                    current_fn_return = 0;
                    assembly.push_str(&format!("({})\n", name));
                    for _ in 0..*no_of_locals {
                        self.load_const_value(0, &mut assembly);
                        self.push_to_segment("@LCL", &mut assembly);
                        self.increase_pointer("@LCL", &mut assembly);
                    }
                }
                Instruction::CallFunc(name, no_of_args) => {
                    let return_address = format!("{}.{}$ret.{}", file_name, current_function, current_fn_return);
                    // push return address (see below) to stack
                    assembly.push_str(&format!("@{}\n", return_address));
                    assembly.push_str("D=A\n");
                    self.push_to_stack(&mut assembly);
                    // push LCL segment address to stack
                    self.load_segment_address("@LCL", &mut assembly);
                    self.push_to_stack(&mut assembly);
                    // push ARG segment address to stack
                    self.load_segment_address("@ARG", &mut assembly);
                    self.push_to_stack(&mut assembly);
                    // push THIS segment address to stack
                    self.load_segment_address("@THIS", &mut assembly);
                    self.push_to_stack(&mut assembly);
                    // push THAT segment address to stack
                    self.load_segment_address("@THAT", &mut assembly);
                    self.push_to_stack(&mut assembly);
                    // calculate ARG
                    let jump_back_by = 5 + no_of_args;
                    self.load_segment_address("@SP", &mut assembly);
                    for _ in 0..jump_back_by {
                        assembly.push_str("D=D-1\n");
                    }
                    assembly.push_str("@ARG\n");
                    assembly.push_str("M=D\n");
                    // calculate new LCL
                    self.load_segment_address("@SP", &mut assembly);
                    assembly.push_str("@LCL\n");
                    assembly.push_str("M=D\n");
                    // goto function
                    assembly.push_str(&format!("@{}\n", name));
                    assembly.push_str("0;JMP\n");
                    assembly.push_str(&format!("({})\n", return_address));
                    current_fn_return += 1;
                }
                Instruction::Return => {
                    // store LCL address temporarily in R13
                    self.load_segment_address("@LCL", &mut assembly);
                    self.push_to_address("@R13", &mut assembly);
                    // calculate return address and store it temporarily in R14
                    self.load_segment_address("@R13", &mut assembly);
                    for _ in 0..5 {
                        assembly.push_str("D=D-1\n");
                    }
                    assembly.push_str("A=D\n");
                    assembly.push_str("D=M\n");
                    self.push_to_address("@R14", &mut assembly);
                    // store return value in ARG
                    self.pop_from_stack(&mut assembly);
                    self.select_mem_indirect("@ARG", &mut assembly);
                    assembly.push_str("M=D\n");
                    self.load_segment_address("@ARG", &mut assembly);
                    assembly.push_str("D=D+1\n");
                    self.push_to_address("@SP", &mut assembly);
                    // restore THAT segment pointer
                    // restore THIS segment pointer
                    // restore ARG segment pointer
                    // restore LCL segment pointer
                    // goto return address
                    self.load_segment_address("@R14", &mut assembly);
                    assembly.push_str("A=D\n");
                    assembly.push_str("0;JMP\n");
                }
            }
            if debug {
                println!("{:?} -> {:?}", instruction, assembly);
            }
            self.file.write_fmt(format_args!("{}\n", assembly))?;
        }
        self.file.flush()
    }

    fn load_const_value(&self, value: u16, assembly: &mut String) {
        assembly.push_str(&format!("@{}\n", value));
        assembly.push_str("D=A\n");
    }

    fn push_to_stack(&self, assembly: &mut String) {
        self.select_mem_indirect("@SP", assembly);
        assembly.push_str("M=D\n");
        self.increase_pointer("@SP", assembly);
    }

    fn pop_from_stack(&self, assembly: &mut String) {
        self.decrease_pointer("@SP", assembly);
        assembly.push_str("A=M\n");
        assembly.push_str("D=M\n");
    }

    fn select_mem_indirect(&self, address: &str, assembly: &mut String) {
        assembly.push_str(&format!("{}\n", address));
        assembly.push_str("A=M\n");
    }

    fn load_segment_address(&self, address: &str, assembly: &mut String) {
        assembly.push_str(&format!("{}\n", address));
        assembly.push_str("D=M\n");
    }

    fn select_segment_index(&self, base: &str, idx: u16, assembly: &mut String) {
        assembly.push_str(&format!("@{}\n", idx));
        assembly.push_str("D=A\n");
        assembly.push_str(&format!("{}\n", base));
        assembly.push_str("M=M+D\n");
    }

    fn deselect_segment_index(&self, base: &str, idx: u16, assembly: &mut String) {
        assembly.push_str(&format!("@{}\n", idx));
        assembly.push_str("D=A\n");
        assembly.push_str(&format!("{}\n", base));
        assembly.push_str("M=M-D\n");
    }

    fn push_to_segment(&self, base: &str, assembly: &mut String) {
        assembly.push_str(&format!("{}\n", base));
        assembly.push_str("A=M\n");
        assembly.push_str("M=D\n");
    }

    fn push_to_address(&self, base: &str, assembly: &mut String) {
        assembly.push_str(&format!("{}\n", base));
        assembly.push_str("M=D\n");
    }

    fn exec_operation(&self, i: usize, op: &Operation, assembly: &mut String) {
        self.decrease_pointer("@SP", assembly);
        assembly.push_str("A=M\n");
        match op {
            Operation::Add => {
                assembly.push_str("D=D+M\n");
            }
            Operation::Sub => {
                assembly.push_str("D=M-D\n");
            }
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
            Operation::And => {
                assembly.push_str("D=D&M\n");
            }
            Operation::Or => {
                assembly.push_str("D=D|M\n");
            }
            Operation::Neg => {
                assembly.push_str("D=-M\n");
            }
            Operation::Not => {
                assembly.push_str("D=!M\n");
            }
        }
    }

    fn increase_pointer(&self, address: &str, assembly: &mut String) {
        assembly.push_str(&format!("{}\n", address));
        assembly.push_str("M=M+1\n");
    }

    fn decrease_pointer(&self, address: &str, assembly: &mut String) {
        assembly.push_str(&format!("{}\n", address));
        assembly.push_str("M=M-1\n");
    }
}
