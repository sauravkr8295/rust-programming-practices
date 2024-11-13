fn first_word(s: &String) ->usize{
    let bytes=s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item==b' '{
            return i;
        }
    }
    s.len()
}

fn main() {
    let val=String::from("This is saurav");
    let res=first_word(&val);
    println!("{res}");
}
