use rand::Rng;

use crate::generator::Generator;
use crate::rule::Rule;

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
            match *l
            {
                'C' => {
                     _letters.push(
                        generator.consonants[rand::rng()
                            .random_range(0..=generator.consonants.len()-1)]);
                    
                }
                'V' => {
                    _letters.push(
                        generator.vowels[rand::rng()
                            .random_range(0..=generator.vowels.len()-1)]);
                }
                'c' => {
                    if rand::rng().random_bool(0.5)
                    {
                         _letters.push(
                            generator.consonants[rand::rng()
                                .random_range(0..=generator.consonants.len()-1)]);
                    }
                }
                'v' => {
                    if rand::rng().random_bool(0.5)
                    {
                        _letters.push(
                            generator.vowels[rand::rng()
                                .random_range(0..=generator.vowels.len()-1)]);
                    }
                }
                _ => {}

            }
        }

        Self 
        { 
            letters: _letters,
        }
    }
}
