use crate::generator::Generator;

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
                "[syllable_structure]" => 
                {
                    _structure = self.parse_structure();
                }
                "[sounds]" => 
                {
                    (_consonants, _vowels) = self.parse_sounds();
                }
                "[rules]" => continue,
                _ => {}
            }
        }

        Generator
        {
            num_syllable: 0,
            structure: _structure,
            separator: '.',
            consonants: _consonants,
            vowels: _vowels
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
}
