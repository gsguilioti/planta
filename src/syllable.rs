use rand::Rng;

use crate::generator::Generator;

pub struct Syllable
{
    pub letters: Vec<char>,
}

impl Syllable
{
    pub fn new(generator: &Generator) -> Self
    {
        let mut _letters: Vec<char> = Vec::new();
        for l in &generator.structure
        {
            if *l == 'C'
            {
                _letters.push(
                    generator.consonants[rand::rng().random_range(0..=generator.consonants.len()-1)]);
            }
            else if *l == 'V'
            {
                _letters.push(
                    generator.vowels[rand::rng().random_range(0..=generator.vowels.len()-1)]);
            }
        }

        Self { letters: _letters}
    }
}
