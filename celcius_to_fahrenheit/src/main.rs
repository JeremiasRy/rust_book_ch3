use std::io;
fn main() {
    println!("Hello and welcome to fahrenheit to celcius converter!");
    
    println!("Please select number 0 for fahrenheit to celcius OR 1 for celcius to fahrenheit!");

    let decision: i8 = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("What is this???");

        let input: i8 = match input
            .trim()
            .parse() {
                Ok(num) => {
                    if num > 1 {
                        println!("Choose 1 or 0"); 
                        continue;
                    }
                    else {
                        num
                    }
                },
                Err(_) => {
                    println!("Please a number!");
                    continue;
                },
           };
        break input;
    };

    if decision == 0 {
        println!("It's {} celcius outside", fahrenheit_to_celcius(get_input(String::from("fahrenheits"))));
    } else {
        println!("It's {} fahrenheit", celcius_to_fahrenheit(get_input(String::from("celcius"))))
    }
}

fn fahrenheit_to_celcius(fahrenheit:f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(celcius:f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}

fn get_input(value_type: String) -> f64 {
    println!("How many {value_type} is it outside?");

    let mut amount = String::new();
    let result: f64 = loop {
        io::stdin()
            .read_line(&mut amount)
            .expect("What is this!!!!");

        let amount:f64 = match amount
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Thats not a temperature!!! Try again");
                    continue;
                },
            };
        break amount;
    };
    result
}

