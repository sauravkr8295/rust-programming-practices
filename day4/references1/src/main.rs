fn main() {
    let s=String::from("hello");
    takes_ownership(s.clone());
    println!("{s}");
    let x=5;
    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(x: i64){
    println!("{x}");
}