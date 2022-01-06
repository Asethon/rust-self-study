use my_project::Phone;
use my_project::Rule;

fn main() {
    let rule_ru = Rule::new("ru", 10, "+7");
    let rule_uk = Rule::new("uk", 10, "+44");
    let rules = vec![rule_ru, rule_uk];
    let numbers = vec!["+79951846979", "+441539612345"];
    let mut phones: Vec<Phone> = vec![];

    for number in numbers {
        for rule in &rules {
            if rule.code == &number[0..rule.code.len()] {
                match Phone::validate(number, rule) {
                    Some(t) => phones.push(t),
                    None => {}
                }
            }
        }
    }

    for phone in phones {
        phone.print();
    }
}
