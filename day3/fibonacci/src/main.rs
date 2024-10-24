use std::io;
fn main() {
    println!("Enter the number:");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("unable to read!");
    let num:i64=num.trim().parse().expect("incorrect input format");
    println!("{}",fibonacci(num));
}
fn fibonacci(n:i64) -> i64{
    if n<=1{
        n
    }
    else{
        fibonacci(n-1)+fibonacci(n-2)
    }   
}