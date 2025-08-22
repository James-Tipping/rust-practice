fn main() {
    let my_string = String::from("Hello World!");

    let first_word = get_first_word_in_string(&my_string);

    println!("Initial string: {my_string}");

    println!("First word in string: {first_word}");
}

fn get_first_word_in_string(s: &String) -> &str {
    let chars = s.as_bytes();

    for (index, &item) in chars.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    return &s[..];
}
