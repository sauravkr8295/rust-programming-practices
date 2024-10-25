fn main() {
    let width=20.0;
    let height=10.0;
    println!("The area of the rectangle is: {} square meter",area_of_rectangle(width,height));
}
fn area_of_rectangle(width: f64 ,height: f64) -> f64{
    width*height
}
