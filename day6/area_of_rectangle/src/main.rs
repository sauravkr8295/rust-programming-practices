struct Rectangle{
    width:f64,
    heigth:f64,
}

fn area_of_rectangle(rectangle:&Rectangle) -> f64{
    rectangle.width * rectangle.heigth
}

fn main() {
    let rectangle=Rectangle{
        width:30.0,
        heigth:20.0,
    };
    println!("{}",area_of_rectangle(&rectangle));
    
}
