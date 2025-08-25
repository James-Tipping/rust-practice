use std::collections::HashMap;

fn main() {
    let mut country_codes: HashMap<String, String> = HashMap::new();

    country_codes.insert(String::from("United Kingdom"), String::from("Unknown"));
    country_codes.insert(String::from("United States"), String::from("1"));
    country_codes.insert(String::from("Singapore"), String::from("65"));
    country_codes.insert(String::from("Ireland"), String::from("353"));

    for (country, code) in &country_codes {
        println!("Country: {}, Code: {}", country, code);
    }

    // Inserting same key will overwrite existing value
    country_codes.insert(String::from("United Kingdom"), String::from("44"));

    country_codes
        .entry(String::from("United Kingdom"))
        .or_insert(String::from("Default"));

    for (country, code) in &country_codes {
        println!("Country: {}, Code: {}", country, code);
    }

    let mut random_map = HashMap::new();
    let text = "hello hello wonderful world";

    for word in text.split_whitespace() {
        let count = random_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Hashmap: {:#?}", random_map);

    // prints:
    // Hashmap: {
    //  "world": 2,
    //  "wonderful": 1,
    //  "hello": 1,
    // }
}
