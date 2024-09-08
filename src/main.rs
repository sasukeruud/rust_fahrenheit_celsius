use std::io;

fn main() {
    println!("Welcome to converter for temperature");

    loop {
        println!("1. convert from fahrenheit to celsius");
        println!("q for quiting");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.as_str() {
            "q" => break,
            _ => continue
        if input.trim() == "1" {
            println!("Input the temperature you want to convert:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");

            let number: i32 = match input.trim().parse() {
                Ok(number) => number,
                Err(_) => continue
            };

            let convert = convert_to_celsius(number);

            println!("Converted {number}F to {convert}C")
        } else if input.trim() == "q" {
            break
        } else {
            println!("Invalid input");
        }
    }
}

fn convert_to_celsius(value: i32) -> i32 {
    (value - 32) * 5 / 9
}