use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {}

fn open_file_match_error_handling() {
    let file_result = File::open("filepath.txt");

    let file = match file_result {
        Ok(file) => {
            println!("File {:#?} opened successfully", file);
            file
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("filepath.txt") {
                Ok(created_file) => {
                    println!("File created");
                    created_file
                }
                Err(e2) => panic!("There was an error creating the file: {}", e2),
            },
            _ => {
                panic!("There was an unknown error opening the file: {}", e);
            }
        },
    };
}

fn open_file_using_unwrap() {
    let file = File::open("filepath.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("filepath.txt").unwrap_or_else(|error| {
                panic!("Error creating file {}", error);
            })
        } else {
            panic!("Error opening file. Error: {}", error);
        }
    });
}

fn open_file_using_question_mark_operator() -> Result<String, io::Error> {
    let mut username_file = File::open("filepath.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}
