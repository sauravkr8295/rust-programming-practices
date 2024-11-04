#[derive(Debug)]
struct Rectangle{
    width: f64,
    heigth: f64,
}

fn main() {
    let rect1=Rectangle{
        width: 20.0,
        heigth: 10.0,
    };
    println!("The area will be: {:?}",rect1);
}
