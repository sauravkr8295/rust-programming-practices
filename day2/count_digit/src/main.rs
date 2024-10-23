use std::io;
fn main() {
    println!("Enter the number:");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("unable to read");
    let num: u32=num.trim().parse().expect("wrong input u entered");
    let res=count_digit(num);
    println!("total {res} digits are there in {num}");
}
fn count_digit(mut num: u32) -> u32{
    let mut count=0;
    loop{
        count=count+1;
        num=num/10;
        if num==0{
            break count;
        }     
    }
}
