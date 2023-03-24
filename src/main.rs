use std::io;

fn main() {

    loop {
        let mut fahrenheit = String::new();

    // Reads the input from STDIN and places it in the String named fahrenheit.
        println!("What's the Fahrenheit temperature?");

        io::stdin().read_line(&mut fahrenheit)
            .expect("Failed to read input.");

    // Convert to an i32, keeps the loop if invalid input.
        let fahrenheit: i32 = match fahrenheit.trim().parse::<i32>() {
            Ok(parsed_fahrenheit) => parsed_fahrenheit,
            Err(_) => continue,
        };
        

        print!("Celsius temperature is {} degrees ", fahrenheit - 17);
       

        println!("when the Fahrenheit temperature is {fahrenheit} degrees");
        break;
    }
}
