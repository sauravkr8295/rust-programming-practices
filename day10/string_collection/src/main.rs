fn main() {

/* 
    
    let s= "Hello S!";
    // let str1=s.to_string();
    let str1="hello S!".to_string();
    println!("{str1}");

*/

/*
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

*/


/*

let mut s = String::from("lo");
s.push_str("l");
println!("{s}");

*/

/*

let mut s1 = String::from("foo");
// let s2 = "bar";
let s2 =String::from("bar");
s1.push_str(&s2);
println!("s2 is {s2}");

*/


let s1 = String::from("Hii, ");
let s2 = String::from("Dear!");
let s3 = s1 + &s2; 

println!("{s3}");





}
