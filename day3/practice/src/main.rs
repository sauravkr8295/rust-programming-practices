use std::io;
fn main(){
    println!("Enter the number:");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("unable to read!");
    let num:u64=num.trim().parse().expect("input is not a number!");
    let res=count_digit(num);
    println!("total {res} digits are there.");
}
fn count_digit(mut num: u64) -> u32 {
    let mut count=0;
    loop{
        count=count+1;
        num=num/10;
        if num==0{
            break count;
        }
    }
}