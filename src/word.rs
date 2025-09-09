use rand::Rng;
use std::fmt::*;
use std::fmt;

use crate::generator::Generator;

pub struct Word
{
    pub letters: Vec<char>,
}

impl Word
{
    pub fn new(generator: &Generator) -> Self
    {
        let word_size = rand::rng().random_range(2..=10);
        let mut _letters: Vec<char> = Vec::new();
        
        for _num in 0..word_size
        {
            let letter_type = rand::rng().random_range(1..=2);
            let consonant = rand::rng().random_range(0..=generator.consonants.len()-1);
            let vowel = rand::rng().random_range(0..=generator.vowels.len()-1);

            if letter_type == 1 
            { _letters.push(generator.consonants[consonant]); }
            else
            { _letters.push(generator.vowels[vowel]); }
        }

        Word
        {
            letters: _letters,
        }
    }
}

impl fmt::Display for Word
{
    fn fmt(&self, f:&mut Formatter) -> fmt::Result
    {
        let s: String = self.letters.iter().collect();
        write!(f, "{}", s)
    }
}
