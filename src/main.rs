use std::io;

fn main() {

    loop {
        let mut fahrenheit = String::new();

    // Reads the input from STDIN and places it in the String named fahrenheit.
        println!("What's the Fahrenheit temperature?");

        io::stdin().read_line(&mut fahrenheit)
            .expect("Failed to read input.");

    // Convert to an i32, keeps the loop going if invalid input.
        let fahrenheit: f64 = match fahrenheit.trim().parse::<f64>() {
            Ok(parsed_fahrenheit) => parsed_fahrenheit,
            Err(_) => {
                println!("please input a valid number");
                continue;
            }
        };

        let celsius = (fahrenheit - 32.1)/1.8;

        print!("Celsius temperature is {celsius} ");
       

        println!("when the Fahrenheit temperature is {fahrenheit} ");
        break;
    }
}
