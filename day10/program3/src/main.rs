#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    
    let row=vec![SpreadsheetCell::Int(3),SpreadsheetCell::Float(30.3),SpreadsheetCell::Text(String::from("I am Saurav")),];
    for i in &row{
        println!("{:?}",i);
    }
}
