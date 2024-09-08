use std::io;

fn main() {
    println!("Welcome to converter for temperature");

    loop {
        println!("1. convert from fahrenheit to celsius");
        println!("2. convert from celsius to fahrenheit");
        println!("q for quiting");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");


        match input.as_str().trim() {
            "1" => {
                println!("Input the temperature you want to convert:");
                for _ in 0..input.len() { input.pop(); }
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");

                let number: i32 = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => continue
                };

                let convert = convert_to_celsius(number);

                println!("Converted {number}F to {convert}C")
            }
            "2" => {
                println!("Input the temperature you want to convert:");
                for _ in 0..input.len() { input.pop(); }
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");

                let number: i32 = match input.trim().parse() {
                    Ok(number) => number,
                    Err(_) => continue
                };

                let convert = convert_to_fahrenheit(number);

                println!("Converted {number}C to {convert}F")
            }
            "q" => break,
            _ => continue
        }
    }
}

fn convert_to_fahrenheit(value: i32) -> i32 {
    value * 9/5 + 32
}

fn convert_to_celsius(value: i32) -> i32 {
    (value - 32) * 5 / 9
}