use rand::Rng;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

// Header.
pub fn print_logo() -> String {
    let logo = r#"
                         88                                                        88  88         
                         ""                                                 ,d     ""  88         
                                                                            88         88         
 ,adPPYba,  8b,dPPYba,   88   ,adPPYb,d8  88,dPYba,,adPYba,   ,adPPYYba,  MM88MMM  88  88   ,d8   
a8P_____88  88P'   `"8a  88  a8"    `Y88  88P'   "88"    "8a  ""     `Y8    88     88  88 ,a8"    
8PP"""""""  88       88  88  8b       88  88      88      88  ,adPPPPP88    88     88  8888[      
"8b,   ,aa  88       88  88  "8a,   ,d88  88      88      88  88,    ,88    88,    88  88`"Yba,   
 `"Ybbd8"'  88       88  88   `"YbbdP"Y8  88      88      88  `"8bbdP"Y8    "Y888  88  88   `Y8a  
                              aa,    ,88                                                          
                               "Y8bbdP"                                                           
"#;

    format!("{}\nWelcome to Enigmatik! 🔐 Secure password generator.\n", logo)
}

// Prompts the user to input their preferences for including .
// symbols and numbers in a password, returning a tuple (bool, bool) representing these options.
pub fn get_user_options() -> (bool, bool) {
    print!("Include symbols? (y/n): ");
    io::stdout().flush().unwrap(); // Ensures the prompt text is immediately shown to the user by flushing the output buffer.
    
    // A new, empty String is created to store the user’s input.
    let mut symbols = String::new();

    // Reads the input from standard input and appends it to the input string.
    // unwrap() is used to handle any errors (though in this context, it will panic if an error occurs).
    io::stdin().read_line(&mut symbols).unwrap();

    // Check if `y` or `n`.
    let symbols = symbols.trim().to_lowercase() == "y";

    print!("Include numbers? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();

    // Check if `y` or `n`.
    let numbers = numbers.trim().to_lowercase() == "y";

    (symbols, numbers)
}

// Prompts the user to input a desired password length, 
// ensures that the input is valid (an integer greater than or equal to 8), 
// and returns the valid password length as a usize.
pub fn get_password_length() -> usize {
    loop {
        print!("Enter desired password length (min 8): ");
        // ensures that the prompt is displayed immediately 
        // by flushing the standard output buffer. 
        // Without this, the text might not appear promptly.
        io::stdout().flush().unwrap();
       
        //
        // Read the user input.
        //

        // A new, empty String is created to store the user’s input.
        let mut input = String::new(); 

        // Reads the input from standard input and appends it to the input string.
        // unwrap() is used to handle any errors (though in this context, it will panic if an error occurs).
        io::stdin().read_line(&mut input).unwrap(); 
        
        // Parse the input
        let length: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parse fails, ask again.
        };
        
        if length >= 8 {
            return length;
        } else {
            println!("😕 Password length must be at least 8 characters. Try again.");
        }
    }
}

// Generates a random password of a specified length, 
// optionally including symbols and numbers. 
// It uses the rand crate for randomness.
pub fn generate_password(length: usize, include_symbols: bool, include_numbers: bool) -> String {
    // Creates a thread-local random number generator using rand::thread_rng(). 
    // This RNG will be used to randomly pick characters for the password.
    let mut rng = rand::thread_rng();

    // Defines a base character set of lowercase and uppercase letters.
    // Converts the string into a vector of characters.
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    
    // Creates a mutable vector, pool, initialized as a copy (clone()) of the base character set chars.
    let mut pool: Vec<char> = chars.clone(); 

    if include_symbols {
        pool.extend("!@#$%^&*()_-+=<>,./?;:".chars());
    }

    if include_numbers {
        pool.extend("0123456789".chars());
    }

    // Generation
    // creates a range from 0 to length - 1
    //
    let password: String = (0..length) // creates a range from 0 to length - 1 
        .map(|_| pool[rng.gen_range(0..pool.len())]) // For each iteration, a random index is generated using rng.gen_range(0..pool.len())
        .collect(); // Gathers all the selected characters into a String.

    // Simulating an animation
    print!("⏳ Generating");
    for _ in 0..5 {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }
    println!();

    password
}
