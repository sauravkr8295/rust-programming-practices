fn find_first_word(s:&str) -> &str{
    let bytes=s.as_bytes();
    for(i,&val) in bytes.iter().enumerate(){
        if val==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}


fn main() {
    let str=String::from("variable is bydefault immutable in rust");
    println!("{}",find_first_word(&str));
}
