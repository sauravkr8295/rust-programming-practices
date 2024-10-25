fn main() {
    let rectangle=(20.0,10.0);
    println!("Area of the triangle is: {}",area_of_rectangle(rectangle));
}
fn area_of_rectangle(rectangle: (f64,f64)) -> f64{
    rectangle.0 * rectangle.1
}
