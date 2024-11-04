#[derive(Debug)]
struct Rectangle{
    width:f64,
    length:f64,
}

impl Rectangle{
    fn area_of_rectangle(&self) -> f64{
        self.width * self.length
    }
    fn is_fit(&self, other:&Rectangle) -> bool{
        self.width>other.width && self.length>other.length
    }
}

fn main() {
    let rect1=Rectangle{
        width:20.0,
        length:10.0,
    };
    let rect2=Rectangle{
        width:10.0,
        length:5.0,
    };
    let rect3=Rectangle{
        width:30.0,
        length:15.0,
    };
    println!("Rect2 will fit to Rect1 or not? {}",rect1.is_fit(&rect2));
    println!("Rect3 will fit to Rect1 or not? {}",rect1.is_fit(&rect3));
}
