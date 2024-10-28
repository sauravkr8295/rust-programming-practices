fn first_word(s: &str) -> &str {
    let bytes=s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    // let s=String::from("Iam saurav");
    let s="sa daff";
    let res=first_word(&s);
    println!("{res}");
}

// By changing the function signature to accept &str rather than &String, the function can now handle both string slices and string literals. 
// The improved function first_word works with both String and &str types, making it more flexible and less error-prone. 
