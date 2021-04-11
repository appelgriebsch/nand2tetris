use std::path::PathBuf;

use code_generator::CodeGen;
use parser::Parser;
use structopt::StructOpt;

mod parser;
mod code_generator;
mod symbol_table;

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
    files: Vec<PathBuf>
}

fn main() {
	let opt = Opt::from_args();
	let output_path = opt.output.to_path_buf();
	for file in opt.files {
		if let Some(f) = file.to_str() {			
			let mut parser = Parser::new(f);			
			match parser.parse_file() {
			    Ok((instructions, symbol_table)) => {
			    	if opt.debug {
			    		for instruction in instructions {
			    			println!("{:?}", instruction);
			    		}
			    		symbol_table.print_symbols();
			    	}
			    	let mut output_file = output_path.clone();
			    	output_file.push(file.file_name().unwrap());
			    	let output_filename = output_file.to_str().unwrap().replace(".asm", ".hack");
			    	let code_gen = CodeGen::new(&output_filename, instructions, symbol_table);
			    	match code_gen.generate(opt.debug) {
			    		Ok(_) => println!("Compiled successfully to {}", output_filename),
			    		Err(e) => println!("Error: {}", e)
			    	}
			    }
			    Err(msg) => {
			    	println!("{:?}", msg);
			    }
			}	
		}
	}
}
