use std::{fs::File, io::{self, Read}};

fn main(){
    let result = read_username_from_file();
    match result {
        Ok(username) => println!("{}", username),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn read_username_from_file()->Result<String,io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}