use std::{fs::{self, File}, io::Write};
use std::path::PathBuf;
use structopt::StructOpt;
use parser::Parser;

mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "jackc")]
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
    output_file.push(format!("{}.vm", opt.file.file_stem().unwrap().to_str().unwrap()));
    output_filename = output_file.to_str().unwrap().to_string();
    if let Ok(entries) = fs::read_dir(opt.file) {
      files = entries.map(|e| e.unwrap().file_name().into_string().unwrap())
                     .filter(|name| name.ends_with(".jack"))
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
    output_filename = opt.file.clone().to_str().unwrap().replace(".jack", ".vm");
  }
  if opt.debug {
    println!("{:?} -> {:?}", files.join(";"), output_filename);
  }
  let mut output_file = File::create(output_filename).unwrap();
  for file in files {
    println!("{:?}", file);
    let mut p = Parser::new(&file);
    match p.get_tokens() {
        Ok(tokens) => {
          output_file.write_fmt(format_args!("<tokens>\n"));
          for token in tokens {
            match token {
                parser::JackToken::Comment(c) => { println!("ignored comment: {:?}", c); }
                parser::JackToken::Keyword(k) => { output_file.write_fmt(format_args!("<keyword>{}</keyword>\n", k)); }
                parser::JackToken::Symbol(s) => { output_file.write_fmt(format_args!("<symbol>{}</symbol>\n", s.replace("\"", ""))); }
                parser::JackToken::IntegerContant(i) => { output_file.write_fmt(format_args!("<integerConstant>{}</integerConstant>\n", i)); }
                parser::JackToken::StringConstant(s) => { output_file.write_fmt(format_args!("<stringConstant>{}</stringConstant>\n", s.replace("\"", ""))); }
                parser::JackToken::Identifier(ident) => { output_file.write_fmt(format_args!("<identifier>{}</identifier>\n", ident.replace("\"", ""))); }
            } 
          }
          output_file.write_fmt(format_args!("</tokens>\n"));
        }
        Err(e) => {
          eprintln!("{:?}", e);
        }
    }
  }
}
