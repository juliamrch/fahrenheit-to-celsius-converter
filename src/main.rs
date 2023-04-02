use std::io;

fn main() {

     println!("=========================================");
     println!("| Let's convert a temperature! |");
     println!("=========================================\n");
    
    loop {
        // Gets input from user
        println!("Please input a temperature");

        let mut temperature = String::new();
        
        // Reads the input
        io::stdin().read_line(&mut temperature)
        .expect("Failed to read line.");

        // Parses the new string to interpret as a f32 number.
        // This is necessary for any input, to perfom mathematic operations from an inputed number.
        let temperature: f32 = match temperature.trim().parse() {
            
            // Makes sure the input is a number
            Ok(num) => num,
            Err(_) => continue
        };
        
        //Returns the converted temperature
        println!("Fahrenheit temperature is {:.1}", celsius_to_fahrenheit(temperature));
        println!("Celsius temperature is {:.1}", fahrenheit_to_celsius(temperature));
    }

}

// Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(temperature: f32) -> f32 {
    let temperature = temperature * 1.8 + 32.0;
    return temperature;
}

// Converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    let temperature = (temperature-32.0) / 1.8;
    return temperature;
}
