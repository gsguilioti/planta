
#[derive(PartialEq)]
pub enum Place
{
    ONSET,
    CODA,
}

pub enum Action
{
    ALLOW{letters: Vec<char>},
    FORBID{letters: Vec<char>},
}

pub enum Func
{
    AFTER{letters: Vec<char>},
    BEFORE{letters: Vec<char>},
    NONE,
}

pub enum Syl
{
    FIRST,
    LAST,
    ANY,
}

pub struct Rule
{
    pub place: Place,
    pub pos: u8,
    pub act: Action,
    pub func: Func,
    pub syl: Syl,
}

impl Rule
{
    pub fn new(rule: &String) -> Rule
    {
        Self
        {
            place: Place::ONSET,
            pos: 0,
            act: Action::ALLOW{letters: Vec::<char>::new()},
            func: Func::AFTER{letters: Vec::<char>::new()},
            syl: Syl::ANY,
        }
    }

    pub fn allow(&self, letter: char, place: &Place) -> bool
    {
        if &self.place != place { return true; }
        
        match &self.act
        {
            Action::ALLOW {letters} => letters.contains(&letter),
            Action::FORBID {letters} => !letters.contains(&letter),
        }
    }
}
