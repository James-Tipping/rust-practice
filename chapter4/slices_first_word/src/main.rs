fn main() {
    let initial_string = String::from("Hello World");

    let first_word_in_string = first_word(&initial_string);

    println!("Initial string is: {initial_string}");
    println!("First word in string is: {first_word_in_string}");
}

fn first_word(string_of_words: &String) -> String {
    let mut char_vector: Vec<char> = vec![];

    for character in string_of_words.chars() {
        if character == char::from(' ') {
            break;
        } else {
            char_vector.push(character);
        }
    }

    char_vector.iter().collect()
}
