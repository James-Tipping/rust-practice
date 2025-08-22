fn main() {
    let mut my_string = String::from("hello");
    let mut my_string_2 = "Hello";

    my_string.push_str(" World");
    my_string_2 = "Hello World";

    // these are not valid as a format string is edxpected
    // println!(my_string);
    // println!(my_string_2);

    let s1 = String::from("String 1");
    let s2 = s1;

    // invalid as invalidated reference
    // println!("{s1}");

    let s3 = s2.clone();

    // both valid as new
    println!("{s2}");
    println!("{s3}");

    let num1 = 27;
    let num2 = num1;

    // valid as integer are stored solely on the stack - known size as compile time
    println!("{num2}");
}
