use rand::Rng;

use crate::generator::Generator;
use crate::rule::{Place, Syl};

pub struct Syllable
{
    pub letters: Vec<char>,
}

impl Syllable
{
    pub fn new(generator: &Generator) -> Self
    {
        let mut processing_at = Place::ONSET;
        let mut _letters: Vec<char> = Vec::new();
        let mut last_letter: char = ' ';

        for l in &generator.structure
        {
            match *l
            {
                'C' => {
                    _letters.push(Self::gen_consonant(generator, &processing_at, &mut last_letter));
                    
                }
                'V' => {
                    _letters.push(
                        generator.vowels[rand::rng()
                            .random_range(0..=generator.vowels.len()-1)]);
                    processing_at = Place::CODA; 
                }
                'c' => {
                    if rand::rng().random_bool(0.5)
                    {
                        _letters.push(Self::gen_consonant(generator, &processing_at, &mut last_letter));
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

    fn gen_consonant(generator: &Generator, place: &Place, last_letter: &mut char) -> char
    {
        loop
        {
            let letter = generator.consonants[rand::rng()
                .random_range(0..=generator.consonants.len()-1)];
            
            if generator.rules.iter().all(|rule| rule.allow(letter, place, *last_letter, &Syl::ANY))
            { 
                *last_letter = letter;
                return letter; 
            }
        }
    }
}
