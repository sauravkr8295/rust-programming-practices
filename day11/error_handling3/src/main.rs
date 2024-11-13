use std::fs::File;
// fn main() {
//     let greeting_file =File::open("hii.txt").unwrap();
// }

fn main() {
    let greeting_file =File::open("hii.txt").expect("hii.txt should be included in this project");
}
