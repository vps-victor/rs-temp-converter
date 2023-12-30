use std::io;

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Function to read user input and parse it into a Result<f64, &'static str>
fn read_input(prompt: &str) -> Result<f64, &'static str> {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        // Read user input from stdin
        io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim().parse() {
            Ok(value) => return Ok(value),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Main function
fn main() {
    // Main loop for the program
    loop {
        println!("Choose the temperature unit:");
        println!("1. Celsius");
        println!("2. Fahrenheit");

        // Read user choice and handle potential errors
        let choice: f64 = match read_input("Enter your choice:") {
            Ok(value) => value,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        // Check if the choice is a valid integer and either 1 or 2
        if choice.fract() == 0.0 && (choice == 1.0 || choice == 2.0) {
            match choice as u32 {
                1 => {
                    // If Celsius is chosen, read Celsius input and convert
                    let celsius = match read_input("Enter the temperature in Celsius:") {
                        Ok(value) => value,
                        Err(err) => {
                            println!("Error: {}", err);
                            continue;
                        }
                    };
                    // Convert and print the result
                    let fahrenheit = celsius_to_fahrenheit(celsius);
                    println!(
                        "{:.2} Celsius is equivalent to {:.2} Fahrenheit",
                        celsius, fahrenheit
                    );
                }
                2 => {
                    // If Fahrenheit is chosen, read Fahrenheit input and convert
                    let fahrenheit = match read_input("Enter the temperature in Fahrenheit:") {
                        Ok(value) => value,
                        Err(err) => {
                            println!("Error: {}", err);
                            continue;
                        }
                    };
                    // Convert and print the result
                    let celsius = fahrenheit_to_celsius(fahrenheit);
                    println!(
                        "{:.2} Fahrenheit is equivalent to {:.2} Celsius",
                        fahrenheit, celsius
                    );
                }
                _ => {
                    println!("Invalid choice. Please choose 1 or 2.");
                }
            }

            // Exit the program after a valid conversion
            break;
        } else {
            // Inform the user about an invalid choice and continue the loop
            println!("Invalid choice. Please enter a valid number (1 or 2).");
        }
    }
}

