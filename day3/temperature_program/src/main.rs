use std::io;
fn main() {
    loop{
        println!("Temperature conversion:");
        println!("Enter 1: Celsius to fahrenheit");
        println!("Enter 2: Fahrenheit to Celsius");
        println!("Enter 3: to Exit");
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("unable to read the input");
        let input:i64=input.trim().parse().expect("Invalid input temperature please enter the valid input");
        if input==3{
            break;
        }
        else if input==1||input==2{
            println!("Enter the temperature:");
            let mut temp=String::new();
            io::stdin().read_line(&mut temp).expect("unable to read input");
            let temp:f64=temp.trim().parse().expect("Incorrect temperature input!");
            match input{
                1 => println!("Fahrenheit value of {} is {}",temp,celsius_to_fahrenheit(temp)),
                2 => println!("Celcius value of {} is {}",temp,fahrenheit_to_celsius(temp)),
                _ => unreachable!(),
            }
        }
        else{
            println!("Invalid option please choose 1,2 or 3.");
        }
    }
}
fn celsius_to_fahrenheit(temp:f64) -> f64{
    (temp*9.0/5.0)+32.0
}
fn fahrenheit_to_celsius(temp:f64) -> f64{
    (temp - 32.0)*5.0/9.0
}

