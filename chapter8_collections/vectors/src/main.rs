fn main() {
    let mut vector1: Vec<i32> = Vec::new();

    vector1.push(1);
    vector1.push(2);
    vector1.push(3);

    let vector2 = vec![1, 2, 3];

    let vector1_final_element = vector1[2];
    println!("vector 1 final element is {}", vector1_final_element);

    let vector2_final_element = vector2.get(2);
    match vector2_final_element {
        Some(vector2_final_element) => {
            println!("vector 2 final element is {}", vector2_final_element)
        }
        None => println!("vector 2 final element is None"),
    }

    let vector3 = vec![String::from("Hello"), String::from("world")];
    // need to create reference as String does not implement copy trait
    // causes a panic as indexing is out of bounds
    // let vector3_final_element = &vector3[2];
    // println!("vector 3 final element is {}", vector3_final_element);

    let mut vector4 = vec![3, 7, 15, 20, 30];
    for i in &vector4 {
        println!("vector4 value: {}", i);
    }

    for i in &mut vector4 {
        *i = *i + 100;
        println!("vector 4 i value is now: {}", i);
    }
}

// using enums to have different data types in a vector
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn get_spreadsheet() -> Vec<SpreadsheetCell> {
    vec![
        SpreadsheetCell::Int(100),
        SpreadsheetCell::Float(0.34),
        SpreadsheetCell::Text(String::from("hello world")),
    ]
}
