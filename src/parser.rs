use crate::generator::Generator;
use crate::rule::*;

#[derive(Default)]
pub struct Config
{
    pub random_syl_num: bool,
    pub num_syllable: u8,
    pub separator: char,
    pub num_words: u16,
}

pub struct Parser
{
    pub content: String,
    pub line: u8,
    pub skip: u8,
}

impl Parser
{
    pub fn new(file_content: &String) -> Self
    {
        Parser
        {
            content: file_content.clone(),
            line: 0,
            skip: 0,
        }
    }

    pub fn parse(&mut self) -> Generator
    {
        let mut _structure: Vec<char> = vec![];
        let mut _consonants: Vec<char> = vec![];
        let mut _vowels: Vec<char> = vec![];
        let mut _rules: Vec<Rule> = vec![];
        let mut config: Config = Config::default();

        let lines: Vec<String> = self.content.lines().map(|s| s.to_string()).collect();

        for (idx, line) in lines.into_iter().enumerate()
        {
            if self.skip > 0
            {
                self.skip -= 1;
                continue;
            }

            if line.is_empty() { continue; }
            
            self.line = idx as u8;

            match line.as_str()
            {
                "[generator]" =>
                {
                    config = self.parse_configs();
                }
                "[syllable_structure]" => 
                {
                    _structure = self.parse_structure();
                }
                "[sounds]" => 
                {
                    (_consonants, _vowels) = self.parse_sounds();
                }
                "[rules]" => _rules = self.parse_rules(),
                _ => {}
            }
        }

        Generator
        {
            num_syllable: config.num_syllable,
            random_syl_num: config.random_syl_num,
            num_words: config.num_words,
            separator: config.separator,
            structure: _structure,
            consonants: _consonants,
            vowels: _vowels,
            rules: _rules,
        }
    }

    fn parse_structure(&mut self) -> Vec<char>
    {
        self.skip = 1;

        self.content.lines().nth((self.line+1).into()).unwrap().chars().collect()
    }

    fn parse_sounds(&mut self) -> (Vec<char>, Vec<char>)
    {
        let mut _consonants: Vec<char> = vec![];
        let mut _vowels: Vec<char> = vec![];
        
        for line in self.content.lines().skip((self.line + 1).into())
        {
            self.skip += 1;

            let line = line.trim();

            if line.starts_with("c:")
            {
                _consonants = line[2..]
                    .split('/')
                    .map(|s| s.trim().chars().next().unwrap())
                    .collect();
            }
            else if line.starts_with("v:")
            {
                _vowels = line[2..]
                    .split('/')
                    .map(|s| s.trim().chars().next().unwrap())
                    .collect();
            }
            else
            {
                break;
            }
        }

        (_consonants, _vowels)
    }

    fn parse_rules(&mut self) -> Vec<Rule>
    {
        let mut _rules: Vec<Rule> = vec![];

        for line in self.content.lines().skip((self.line + 1).into())
        {
            let place: Place;
            let mut act: Action;
            let mut func: Func = Func::NONE;
            let mut syl: Syl = Syl::ANY;

            self.skip += 1;

            let mut line = line.trim();
            
            if line.chars().nth(0) == Some('o') {place = Place::ONSET;}
            else if line.chars().nth(0) == Some('c') {place = Place::CODA;}
            else {break;}

            line = &line[2..].trim();
            if let Some(c) = line.chars().nth(0)
            {
                if c.is_alphabetic()
                {
                    if line.starts_with("ls")
                    {syl = Syl::LAST;}
                    else if line.starts_with("fs")
                    {syl = Syl::FIRST;}

                    line = &line[2..];
                }
            }

            if line.chars().nth(0) == Some('+')
            {act = Action::ALLOW { letters: Vec::new()};}
            else if line.chars().nth(0) == Some('-')
            {act = Action::FORBID {letters: Vec::new()};}
            else {break;}

            let mut end = line.find(')').unwrap();
            let line_aux = &line[2..end];

            match &mut act 
            {
                Action::ALLOW { letters} | Action::FORBID { letters } =>
                {
                    *letters = line_aux.split('/')
                        .filter_map(|s| s.chars().next())
                        .collect();
                }
            }

            line = &line[end+1..];

            if let Some(c) = line.chars().nth(0)
            {
                if c.is_alphabetic()
                {
                    if c == 'a'
                    {func = Func::AFTER {letters: Vec::new()};}
                    else if c == 'b'
                    {func = Func::BEFORE {letters: Vec::new()};}
                    else {break;}

                    end = line.find(')').unwrap();
                    let line_aux = &line[2..end];

                    match &mut func 
                    {
                        Func::AFTER { letters} | Func::BEFORE { letters } =>
                        {
                            *letters = line_aux.split('/')
                                .filter_map(|s| s.chars().next())
                                .collect();
                        },
                        _ => ()
                    }

                }
            }           

            let rule = Rule
                {
                    place: place,
                    act: act,
                    func: func,
                    syl: syl,
                };
            _rules.push(rule);
        }

        _rules
    }

    fn parse_configs(&mut self) -> Config
    {
        let mut _random_syl_num: bool = false;
        let mut _num_syllable: u8 = 0;
        let mut _separator: char = '.';
        let mut _num_words: u16 = 10;


        for line in self.content.lines().skip((self.line + 1).into())
        {
            self.skip += 1;

            let mut line = line.trim();

            if line == "random_syl_num"
            { 
                _random_syl_num = true; 
            }
            else if line.starts_with("num_syllable")
            {
                _num_syllable = line.split_whitespace().nth(1).unwrap().parse::<u8>().unwrap();
            }
            else if line.starts_with("num_words")
            {
                _num_words = line.split_whitespace().nth(1).unwrap().parse::<u16>().unwrap();
            }
            else if line.starts_with("separator")
            {
                _separator = line.split_whitespace().nth(1).unwrap().parse::<char>().unwrap();
            }
            else
            { break; }
        }

        Config
        {
            random_syl_num: _random_syl_num,
            num_syllable: _num_syllable,
            separator: _separator,
            num_words: _num_words,
        }
    }
}
