fn main() {
    let currency = Cash::Tenner(Monarch::Elizabeth);
    let cash_value = match_cash_to_value(&currency);

    println!("The cash value of a {:#?} is {}", currency, cash_value);

    println!("None plus 1 is: {:?}", add_one_to_value(None));
    println!("1 plus 1 is: {:?}", add_one_to_value(Some(1)));
}

#[derive(Debug)]
enum Cash {
    Penny(Monarch),
    Pound(Monarch),
    Fiver(Monarch),
    Tenner(Monarch),
}

#[derive(Debug)]
enum Monarch {
    Elizabeth,
    Charles,
}

fn match_cash_to_value(cash: &Cash) -> u16 {
    match cash {
        Cash::Penny(monarch) => 1,
        Cash::Pound(monarch) => 100,
        Cash::Fiver(monarch) => 500,
        Cash::Tenner(monarch) => {
            println!("Tenner has monarch: {:#?}", monarch);
            // invalid as value partially borrowed - Monarch does no implement copy trait
            // valid when method uses references instead of transferring ownership
            println!("Currency is: {:#?}", cash);
            1000
        }
    }
}

fn add_one_to_value(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}
