#[derive(Debug)]
struct Rectangle{
    width: f64,
    heigth: f64,
}

impl Rectangle{
    fn area_of_rectangle(&self) -> f64{
        self.width * self.heigth
    }
}

fn main() {
    let rect1=Rectangle{
        width:20.0,
        heigth:10.0,
    };
    println!("{:#?}",rect1.area_of_rectangle());
}
