use my_project::Rule;

fn main() {
    let rule_ru = Rule::new("ru", 10, "+7");
    let rule_uk = Rule::new("uk", 10, "+44");
    let rules = vec![rule_ru, rule_uk];
    let numbers = vec!["+79951846979", "+441539612345"];
    let phones = my_project::create_phones(&numbers, &rules);

    for phone in phones {
        phone.print();
    }
}
