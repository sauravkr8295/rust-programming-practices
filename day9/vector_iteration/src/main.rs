// fn main() {
//     let v=vec![1,2,3,4,5];
//     println!("The elements are: ");
//     for i in &v{
//         println!("{i}");
//     }
// }


fn main() {
    let mut v=vec![1,2,3,4,5];
    println!("The elements are after adding 5 in each: ");
    for i in &mut v{
        // *i=*i+5;
        *i+=5;
        println!("{i}");
    }
}
