fn main() {
    let s1=gives_ownership();
    let s2=String::from("Saurav");
    let s3=takes_ownership(s2);
    println!("{s1}");
    println!("{s3}");

}
fn gives_ownership() -> String{
    let s4=String::from("S");
    s4
}

fn takes_ownership(str: String) -> String{
    str
}