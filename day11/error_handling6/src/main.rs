use std::fs::File;
use std::io::{self,Read};

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username =String::new();

    File::open("hii.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    read_username_from_file() ;

}



