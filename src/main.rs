#[derive(Debug)]
struct Phone<'a> {
    number: &'a str,
    rule: &'a Rule
}

#[derive(Debug)]
struct Rule {
    abbreviation: &'static str,
    len: i8,
    code: &'static str,
}

impl Phone<'_> {
    pub fn validate<'a>(number: &'a str, rule: &'a Rule) -> Option<Phone<'a>>
    {
        return if number.len() - rule.code.len() == rule.len as usize {
            Some(Phone { number, rule })
        } else {
            None
        };
    }

    pub fn print(self) {
        println!("Phone number: {}", self.number);
        println!("Country abbreviation of number: {}", self.rule.abbreviation);
        println!("Number len: {}", self.number.len());
        println!("Number code: {}", self.rule.code);
        println!();
        println!();
        println!();
    }
}

fn main() {
    let rule_ru = Rule { abbreviation: "ru", len: 10, code: "+7" };
    let rule_uk = Rule { abbreviation: "uk", len: 10, code: "+44" };
    let rules = vec![rule_ru, rule_uk];
    let numbers = vec!["+79951846979", "+441539612345"];
    let mut phones: Vec<Phone> = vec![];

    for number in numbers {
        let mut phone: Option<Phone>;
        for rule in &rules {
            phone = if rule.code == &number[0..rule.code.len()] {
                Some(Phone::validate(number, rule).unwrap())
            } else {
                None
            };

            match phone {
                Some(t) => {
                    phones.push(t);
                },
                None => {}
            }
        }
    }

    for phone in phones {
        Phone::print(phone);
    }
}
