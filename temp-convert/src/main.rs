use std::io;

fn main() {
    println!("Convert between Fahrenheit and Celcius");
    println!("1. Fahrenheit -> Celcius");
    println!("2. Celcius -> Fahrenheit");
    println!("Mode: ");
    let mut mode = String::new();

    io::stdin().read_line(&mut mode).expect("read mode error");

    let mode: u32 = mode.trim().parse().expect("convert mode value error");

    match mode {
        1 => {
            println!("Input Fahrenheit:");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("read temp error");

            let temp: f64 = temp.trim().parse().expect("convert temp error");
            let celcius_temp: f64 = (temp - 32_f64) * (5_f64 / 9_f64);

            println!("Celcius = {:.2}", celcius_temp)
        }
        2 => {
            println!("Input Celcius:");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("read temp error");

            let temp: f64 = temp.trim().parse().expect("convert temp error");
            let fahrenheit_temp: f64 = temp * (9_f64 / 5_f64) + 32_f64;

            println!("Fahrenheit = {:.2}", fahrenheit_temp)
        }
        _ => panic!("invalid mode value"),
    }
}
