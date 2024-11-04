use std::io;

fn area_of_rectangle(tuple :(u64,u64)) -> u64{
    tuple.0 * tuple.1
}


fn main() {
    println!("Enter the width of rectangle: ");
    let mut width=String::new();
    io::stdin().read_line(&mut width).expect("unable to read the input");
    let width:u64 =width.trim().parse().expect("incorrect input format!");

    println!("Enter the length of rectangle: ");
    let mut length=String::new();
    io::stdin().read_line(&mut length).expect("unable to read the input");
    let length:u64 =length.trim().parse().expect("incorrect input format!");
    let tuple=(width,length);
    println!("The area of the rectangle will be: {}",area_of_rectangle(tuple));

}
