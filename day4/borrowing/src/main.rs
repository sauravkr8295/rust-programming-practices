fn main() {
    let str1=String::from("Saurav");
    let length=cal_length(&str1);
    println!("The length of the String {str1} is {length}");
}
fn cal_length(str1: &String) -> usize {
    str1.len()
}
