use std::path::PathBuf;
use structopt::StructOpt;

use code_generator::CodeGen;
use parser::Parser;

mod code_generator;
mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "hackasm")]
struct Opt {
  // A flag, true if used in the command line. Note doc comment will
  // be used for the help message of the flag. The name of the
  // argument will be, by default, based on the name of the field.
  /// Activate debug mode
  #[structopt(short, long)]
  debug: bool,

  /// Output directory
  #[structopt(short, long, parse(from_os_str))]
  output: PathBuf,

  /// Files to process
  #[structopt(name = "FILE", parse(from_os_str))]
  files: Vec<PathBuf>,
}

fn main() {
  let opt = Opt::from_args();
  for file in opt.files {
    if let Some(f) = file.to_str() {
      let mut parser = Parser::new(f);
      match parser.parse_file() {
        Ok(instructions) => {
          if opt.debug {
            for instruction in instructions {
              println!("{:?}", instruction);
            }
          }
          let mut output_file = opt.output.clone();
          output_file.push(file.file_name().unwrap());
          let output_filename = output_file.to_str().unwrap().replace(".vm", ".asm");
          let code_gen = CodeGen::new(&output_filename, instructions);
          code_gen.generate(opt.debug);
        },
        Err(msg) => {
          eprintln!("ERROR: {:?}", msg);
        }
      }
    }
  }
}
