use std::fs::File;
use std::io::Read;

mod generator;
mod word;
use crate::generator::Generator;

fn main() -> std::io::Result<()>
{
    let mut lang_file = File::open("example/example.lang")?;
    let mut lang_contents = String::new(); 
    lang_file.read_to_string(&mut lang_contents)?;
    
    let generator = Generator::new();

    generator.gen_words();

    Ok(())
}
