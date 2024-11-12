use std::io;

fn sum_of_digit(mut num:i32) -> i32 {
    let mut sum=0;
    while num!=0 {
        let d=num%10;
        sum+=d;
        num/=10;
    }
    sum
}

fn main() {
    println!("Enter the number: ");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("unable to read the input!");
    let num:i32 =num.trim().parse().expect("incorrect input format!");
    let res=sum_of_digit(num);
    println!("Sum of Digit of this number is: {}",res);
}
