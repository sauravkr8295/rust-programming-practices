use std::collections::HashMap;
fn main() {
    let mut scores=HashMap::new();

    let blue=String::from("Blue");
    let yellow=String::from("Yellow");
    // scores.insert(String::from("Blue"),10);
    // scores.insert(String::from("Yellow"),50);

    // scores.insert(blue,10);
    // scores.insert(yellow,50);

    scores.insert(blue.clone(),yellow.clone());
    scores.insert(yellow,blue);

    for (key,value) in &scores{
        println!("{key}: {value}");
    }

}
