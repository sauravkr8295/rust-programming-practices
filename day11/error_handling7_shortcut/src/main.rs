use std::fs;
use std::io;
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    read_username_from_file();
}


//fs::read_to_string function that opens the file, creates a new String, reads the contents of the file, puts the contents into that String, and returns it.