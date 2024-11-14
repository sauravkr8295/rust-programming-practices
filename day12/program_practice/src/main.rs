fn main() {
    let num = 10;
    let reference = &num; 
    println!("The number is: {}", reference); 

    let mut value = 5;
    increment(&mut value); 
    println!("Incremented value: {}", value);
}

fn increment(val: &mut i32) {
    *val += 1; 
}