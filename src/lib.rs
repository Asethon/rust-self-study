pub struct Phone<'a> {
    pub number: &'a str,
    pub rule: &'a Rule<'a>
}

pub struct Rule<'a> {
    pub abbreviation: &'a str,
    pub len: usize,
    pub code: &'a str,
}

impl Rule<'_> {
    pub fn new<'a>(abbreviation: &'a str, mut len: usize, code: &'a str) -> Rule<'a> {
        len += code.len();
        Rule {abbreviation, len, code}
    }
}

impl Phone<'_> {
    pub fn validate<'a>(number: &'a str, rule: &'a Rule<'a>) -> Option<Box<Phone<'a>>>
    {
        return if number.len() == rule.len as usize {
            Some(Box::new(Phone { number, rule }))
        } else {
            None
        };
    }

    pub fn print(&self) {
        println!("Phone number: {}", self.number);
        println!("Country abbreviation of number: {}", self.rule.abbreviation);
        println!("Number len: {}", self.number.len());
        println!("Number code: {}", self.rule.code);
        println!();
        println!();
        println!();
    }
}

pub fn create_phones<'a>(numbers: &'a Vec<&'a str>, rules: &'a Vec<Rule<'a>>) -> Vec<Phone<'a>> {
    let mut phones: Vec<Phone> = Vec::new();
    for number in numbers {
        for rule in rules {
            if rule.code == &number[0..rule.code.len()] {
                match Phone::validate(number, rule) {
                    Some(t) => phones.push(*t),
                    None => {}
                }
            }
        }
    }
    phones
}