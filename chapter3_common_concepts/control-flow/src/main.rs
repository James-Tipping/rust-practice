fn main() {
    let my_number = 9;

    let my_boolean = true;

    if my_number % 3 == 0 {
        println!("{my_number} is divisible by 3");
    } else if my_number % 4 == 0 {
        println!("{my_number} is divisible by 4")
    }

    let my_second_number = if my_boolean { 6 } else { 0 };
    println!("The value of my_second_number is {my_second_number}");

    loop {
        println!("Repeat the text again!");
    }
}
