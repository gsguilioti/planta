use crate::word::Word;
use crate::rule::Rule;

pub struct Generator
{
    pub num_syllable: u8,
    pub structure: Vec<char>,
    pub separator: char,
    pub consonants: Vec<char>,
    pub vowels: Vec<char>,
    pub rules: Vec<Rule>,
}

impl Generator
{
    pub fn new() -> Self
    {
        Generator
        {
            num_syllable: 3,
            structure: vec!['C', 'V', 'C'],
            separator: '.',
            consonants: vec![
                'p','b','t','d','ʈ','ɖ','c','ɟ','k','ɡ','q','ɢ','ʡ','ʔ',
                'm','ɱ','n','ɳ','ɲ','ŋ','ɴ',
                'ʙ','r','ʀ',
                'ⱱ','ɾ','ɽ',
                'ɸ','β','f','v','θ','ð','s','z','ʃ','ʒ','ʂ','ʐ','ɕ','ʑ','ç','ʝ','x','ɣ','χ','ʁ','ħ','ʕ','h','ɦ',
                'ɬ','ɮ',
                'ʋ','ɹ','ɻ','j','ɰ',
                'l','ɭ','ʎ','ʟ',
                'ʘ','ǀ','ǃ','ǂ','ǁ','ɓ','ɗ','ʄ','ɠ','ʛ'
            ],
            vowels: vec![
                'i', 'y', 'ɨ', 'ʉ', 'ɯ', 'u',
                'ɪ', 'ʏ', 'ʊ',
                'e', 'ø', 'ɘ', 'ɵ', 'ɤ', 'o',
                'ə',
                'ɛ', 'œ', 'ɜ', 'ɞ', 'ʌ', 'ɔ',
                'æ', 'ɐ',
                'a', 'ɶ', 'ɑ', 'ɒ',
            ],
            rules: Vec::new(),
        }
    }

    pub fn gen_words(&self)
    {

        for _i in 1..10
        {
            let word = Word::new(self); 
            println!("{0} > {1}", word.word_string, word.sep_string);
        }
    }
}
