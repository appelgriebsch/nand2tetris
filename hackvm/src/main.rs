use std::fs;
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

  /// File to process
  #[structopt(name = "FILE", parse(from_os_str))]
  file: PathBuf,
}

fn main() {
  let opt = Opt::from_args();
  let mut files: Vec<String> = Vec::new();
  let output_filename: String;
  if opt.file.is_dir() {
    let path = opt.file.clone();
    let mut output_file = opt.file.clone();
    output_file.push(format!("{}.asm", opt.file.file_stem().unwrap().to_str().unwrap()));
    output_filename = output_file.to_str().unwrap().to_string();
    if let Ok(entries) = fs::read_dir(opt.file) {
      files = entries.map(|e| e.unwrap().file_name().into_string().unwrap())
                     .filter(|name| name.ends_with(".vm"))
                     .map(|file| {
                        let mut infile = path.clone();
                        infile.push(file);
                        infile.to_str().unwrap().to_string()
                     })
                     .collect();
    }
  }
  else {
    files.push(opt.file.to_str().unwrap().to_owned());
    output_filename = opt.file.clone().to_str().unwrap().replace(".vm", ".asm");
  }
  if opt.debug {
    println!("{:?} -> {:?}", files.join(";"), output_filename);
  }
  let mut code_gen = CodeGen::new(&output_filename);
  code_gen.init().expect("ERROR: Unable to write bootstrap code.");
  for file in files {
    let mut parser = Parser::new(&file);
    match parser.parse_file() {
      Ok(instructions) => {
        if opt.debug {
          for instruction in instructions {
            println!("{:?}", instruction);
          }
        }
        match code_gen.generate(instructions, opt.debug) {
          Ok(_) => println!("Compiled {} successfully into {}", file, output_filename),
          Err(e) => eprintln!("ERROR: {}", e),
        }
      },
      Err(msg) => {
        eprintln!("ERROR: {:?}", msg);
      }
    }
  }
}
