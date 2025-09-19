
#[derive(PartialEq, Debug)]
pub enum Place
{
    ONSET,
    CODA,
}

#[derive(Debug)]
pub enum Action
{
    ALLOW{letters: Vec<char>},
    FORBID{letters: Vec<char>},
}

#[derive(Debug)]
pub enum Func
{
    AFTER{letters: Vec<char>},
    BEFORE{letters: Vec<char>},
    NONE,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Syl
{
    FIRST,
    LAST,
    ANY,
}

#[derive(Debug)]
pub struct Rule
{
    pub place: Place,
    pub act: Action,
    pub func: Func,
    pub syl: Syl,
}

impl Rule
{
    pub fn new() -> Rule
    {
        Self
        {
            place: Place::ONSET,
            act: Action::ALLOW{letters: Vec::<char>::new()},
            func: Func::AFTER{letters: Vec::<char>::new()},
            syl: Syl::ANY,
        }
    }

    pub fn allow(&self, letter: char, place: &Place, last_letter: char, syl: &Syl) -> bool
    {
        if letter == last_letter { return false; }
        if &self.place != place { return true; }
        if &self.syl != syl { return true; }
        
        match &self.act
        {
            Action::ALLOW {letters} => 
            {
                match &self.func
                {
                    Func::BEFORE {letters: fletters} => 
                    {
                        if  fletters.contains(&letter) &&
                            !letters.contains(&last_letter)
                        {return false;}

                        return true;
                    },
                    Func::AFTER {letters: fletters} => 
                    {
                        if  letters.contains(&letter) &&
                            !fletters.contains(&last_letter)
                        {return false;}

                        return true;
                    },
                    Func::NONE => return true,
                }
                ;
            }
            Action::FORBID {letters} => {
                !letters.contains(&letter)}
        }
    }
}
