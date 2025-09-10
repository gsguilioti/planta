use std::fmt::*;
use std::fmt;

use crate::generator::Generator;
use crate::syllable::Syllable;

pub struct Word
{
    pub word_string: String,
    pub syllables: Vec<Syllable>,
}

impl Word
{
    pub fn new(generator: &Generator) -> Self
    {
        let mut _syllables: Vec<Syllable> = Vec::new();
        
        for _num in 0..generator.num_syllable
        {
            _syllables.push(Syllable::new(generator));
        }

        let stringfy: String =  _syllables
                                .iter()
                                .map(|syl| syl.letters.iter().collect::<String>())
                                .collect::<Vec<_>>()
                                .join(&generator.separator.to_string());
        
        Word
        {
            word_string: stringfy,
            syllables: _syllables,
        }
    }
}

impl fmt::Display for Word
{
    fn fmt(&self, f:&mut Formatter) -> fmt::Result
    {
        write!(f, "{}", self.word_string)
    }
}
