use std::env;
use std::fs::File;
use std::io::Read;

mod generator;
mod word;
mod syllable;
mod parser;
mod rule;
use crate::parser::Parser;

fn main() -> std::io::Result<()>
{
    let args: Vec<String> = env::args().collect();

    let mut lang_file = File::open(args[1].clone())?;
    let mut lang_contents = String::new(); 
    lang_file.read_to_string(&mut lang_contents)?;
    
    let mut parser = Parser::new(&lang_contents);
    let generator = parser.parse();

    generator.gen_words();
        
    Ok(())
}
