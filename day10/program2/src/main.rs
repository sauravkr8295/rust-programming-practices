fn main() {
    let mut v:Vec<i32> = Vec::new();
    // let mut v=vec![10,15,12];
    v.push(10);
    v.push(15);
    v.push(12);

/*

    // Fetching the value by indexing.
    let second: &i32 =&v[1];
    println!("Value at the second index is: {second}");

    // Fetching the value by get method.
    let second :Option<&i32> =v.get(1);

    match second{
        Some(second) => println!("The second element is: {second}"),
        None => println!("There is not second element!"),
    }

*/

    let mut v =vec![1,2,3,4,5];
    // let first=&v[0];
    v.push(10);
    // println!("{first}");
    let six=&v[5];
    println!("{six}");

    for i in &mut v{
        *i +=50;
        println!("{i}");
    }

}
