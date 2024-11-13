use std::fs::File;
fn main() {
    let greeting_file_result=File::open("hii.text");
    let greeting_file=match greeting_file_result{
        Ok(file) =>file,
        Err(error) =>panic!("Problem opening the file: {error:?}"),
    };
}
