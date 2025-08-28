fn main() {
    let string1 = String::from("hello");
    let result;

    {
        // this will not affect compilation as it's a static reference
        // let string2 = "world";

        let string2 = String::from("world");

        // this will compile if string2 is static reference as compiler uses shortest of lifetimes & result is not used outside of
        // inner scope
        // let result = get_longest_string(&string1, string2);

        // string 2 does not live long enough
        result = get_longest_string(&string1, &string2);
        println!("longest string is: {:?}", result);
    }
    // this prevents compilation as string2 does not live long enough
    // println!("longest string is: {:?}", result);

    //string2 is out of scope so cannot be accessed
    // println!("{}", string2);
}

fn get_longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
