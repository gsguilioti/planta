use std::fmt::*;
use std::fmt;
use rand::Rng;

use crate::generator::Generator;
use crate::syllable::Syllable;

pub struct Word
{
    pub word_string: String,
    pub sep_string: String,
    pub syllables: Vec<Syllable>,
}

impl Word
{
    pub fn new(generator: &Generator) -> Self
    {
        let mut _syllables: Vec<Syllable> = Vec::new();
       
        let max_syl =   if generator.num_syllable > 0 
                        { rand::rng().random_range(1..=generator.num_syllable) }
                        else
                        { rand::rng().random_range(1..= 3) };

        for _num in 0..max_syl
        {
            _syllables.push(Syllable::new(generator));
        }

        let sep_stringfy: String =  _syllables
                                .iter()
                                .map(|syl| syl.letters.iter().collect::<String>())
                                .collect::<Vec<_>>()
                                .join(&generator.separator.to_string());
        
        let stringfy: String =  _syllables
                                .iter()
                                .map(|syl| syl.letters.iter().collect::<String>())
                                .collect::<String>();
        Word
        {
            word_string: stringfy,
            sep_string: sep_stringfy,
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
