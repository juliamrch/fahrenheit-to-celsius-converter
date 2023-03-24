use std::io;

fn main() {

    loop {
        let mut fahrenheit = String::new();

    // Reads the input from STDIN and places it in the String named fahrenheit.
        println!("What's the Fahrenheit temperature?");

        io::stdin().read_line(&mut fahrenheit)
            .expect("Failed to read input.");

    // Convert to an i32, keeps the loop going if invalid input.
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a valid temperature");
                continue;
            }
        };
        
        //Add a . after the number when using f64 format

        print!("Celsius temperature is {} ", (fahrenheit - 32.)/1.8); //Add a . after the number when using f64 format
       

        println!("when the Fahrenheit temperature is {fahrenheit} ");
        break;
    }
}
