use std::io;
fn main() {
    println!("Enter the String:");
    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("Unable to read the input.");
    let s=s.trim(); // "This is Saurav"
    // println!("{}",s);
    let res=first_word(&s); 
    println!("The first word is: {res}"); 
}

fn first_word(s : &String) -> usize {
    let bytes=s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
