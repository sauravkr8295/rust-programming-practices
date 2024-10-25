struct Rectangle{
    width : u64,
    height :u64,
}

fn main() {
    let rectangle=Rectangle{
        width:20,
        height:10,
    };
    println!("the area of the rectangle is: {}",area_of_rectangle(&rectangle));
}

fn area_of_rectangle(rectangle : &Rectangle) -> u64{
    rectangle.width * rectangle.height
}


