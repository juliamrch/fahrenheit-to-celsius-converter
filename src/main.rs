use std::io;

fn main() {

     println!("=========================================");
     println!("| Let's convert a temperature! |");
     println!("=========================================\n");
    
     println!("Unofficially sponsored by");
     println!(r"  .-')       ('-.      ('-.          .-') _           _ (`-.     ('-.                             
11 +      ( OO ).   _(  OO)    ( OO ).-.     ( OO ) )         ( (OO  )   ( OO ).-.                         
12 +     (_)---\_) (,------.   / . --. / ,--./ ,--,'         _.`     \   / . --. /  ,--. ,--.    ,--.      
13 +     /    _ |   |  .---'   | \-.  \  |   \ |  |\        (__...--''   | \-.  \   |  | |  |    |  |.-')  
14 +     \  :` `.   |  |     .-'-'  |  | |    \|  | )        |  /  | | .-'-'  |  |  |  | | .-')  |  | OO ) 
15 +      '..`''.) (|  '--.   \| |_.'  | |  .     |/         |  |_.' |  \| |_.'  |  |  |_|( OO ) |  |`-' | 
16 +     .-._)   \  |  .--'    |  .-.  | |  |\    |          |  .___.'   |  .-.  |  |  | | `-' /(|  '---.' 
17 +     \       /  |  `---.   |  | |  | |  | \   |          |  |        |  | |  | ('  '-'(_.-'  |      |  
18 ~      `-----'   `------'   `--' `--' `--'  `--'          `--'        `--' `--'   `-----'     `------'  ");
    
    loop {
        // Display conversion types
        println!("============================");
        println!("I got the right temperature to shelter you from the storm, baby gal oh");
        println!("Choose your conversion type");
        println!("============================");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("===========================");
        println!("0. EXIT");

        // Set an index from 0 to 2 to limit user inputs
        // and match the index with conversion variables
        let conversion_type = [0, 1, 2];
        let fahrenheit_to_celsius = conversion_type[1];
        //let celsius_to_fahrenheit = conversion_type[2]; // unused in this scope
        let exit = conversion_type[0];
        
        // Get user desired conversion type
        let mut choice = String::new();
        
        // Read user input
        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

        // Make sure it's a number and that it matches the index
        let choice: usize = choice
        .trim()
        .parse()
        .expect("Please enter a number from 0 to 2");


        // Set 2 conditions depending on user input
        if choice == exit {
            println!("Byyyyyyye, Felicia 👋 👋 👋");
            break;
        } else {
            
            println!("---------------------------");
            println!("Please input a temperature");
        }
        
        // Get user temperature
        let mut temperature = String::new();

        // Read user input
        io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

        // Parse string to be able to perfom a numeric operation on it
        let temperature: f32 = match temperature.trim().parse() {
            
            // Make sure the input is a number
            Ok(num) => num,
            Err(_) => continue
        };

       
            // Permorm the conversion according to matching variables
            if choice == fahrenheit_to_celsius {
            let converted_temperature = (temperature-32.0) / 1.8;
            println!("I got the remedy to make you de-stress out....");
            println!("Your temperature is {:.1}° Celsius", converted_temperature);

            } else {
                let converted_temperature = temperature * 1.8 + 32.0;
                println!("Bumper exposed and gal you got your chest out but you no wasters cause gal");
                println!("your temperature is {:.1}° Fahrenheit", converted_temperature);
            } 
       };

        

    

    }



