fn main() {
    let str=dangling_pointer();
    println!("{str}")
}
// fn dangling_pointer() -> &String {
//     let str=String::from("sau");
//     &str
// }


fn dangling_pointer() -> String {
    let str=String::from("sau");
    str
}
