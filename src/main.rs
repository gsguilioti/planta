use std::fs::File;
use std::io::Read;

mod generator;
mod word;
mod syllable;
mod parser;
use crate::parser::Parser;

fn main() -> std::io::Result<()>
{
    let mut lang_file = File::open("example/example.lang")?;
    let mut lang_contents = String::new(); 
    lang_file.read_to_string(&mut lang_contents)?;
    
    let mut parser = Parser::new(&lang_contents);
    let generator = parser.parse();

    generator.gen_words();

    Ok(())
}
