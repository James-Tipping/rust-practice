use std::{fs::File, io::ErrorKind};

fn main() {
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
