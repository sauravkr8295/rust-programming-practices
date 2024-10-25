fn main() {
    let mut name=String::from("Saurav");
    change_name(&mut name);
    println!("name changed to {name}");
}

fn change_name(name : &mut String) {
    name.push_str(" kumar");
}