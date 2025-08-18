fn main() {
    let string1 = String::from("Hello");

    let string1_length: usize = get_string_length(&string1);

    println!("string length: {string1_length}");

    let my_int: i32 = 3;

    let new_my_int = change_int_value(my_int);

    // this works as int has the copy trait - lives only on the stack
    println!("my_int: {my_int}");
}

fn change_int_value(changing_int: i32) -> i32 {
    let changing_int = 5;

    changing_int
}

fn get_string_length(test_string: &String) -> usize {
    return test_string.len();
}

// Doesn't compile as rust doesn't allow dangling references
// fn return_dangling_reference -> &String() {
//     let my_string = String::from("Hello");
//
//     &my_string
// }
