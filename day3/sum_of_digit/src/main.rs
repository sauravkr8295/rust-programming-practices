use std::io;
fn main() {
    println!("Enter the number:");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("unable to read the input");
    let num=num.trim().parse().expect("Entered input is not a number!");
    let result:u32=sum_of_digit(num);
    println!("the sum of digit of the number is: {result}");
}
fn sum_of_digit(mut num:u32) -> u32{
    let mut sum=0;
    loop{
        let d=num%10;
        sum=sum+d;
        num=num/10;
        if num==0{
            break sum;
        }
    }
}

