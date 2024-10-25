fn main() {
    let str1=String::from("Saurav");
    let (str2,length)=cal_length(str1);
    println!("The length of the string {str2} is {length}");
}

fn cal_length(str1 : String) -> (String,usize)  {
    let length=str1.len();
    (str1,length)
}
