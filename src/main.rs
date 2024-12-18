use rand::Rng;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn print_logo() -> String {
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

fn get_user_options() -> (bool, bool) {
    print!("Include symbols? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut symbols = String::new();
    io::stdin().read_line(&mut symbols).unwrap();
    let symbols = symbols.trim().to_lowercase() == "y";

    print!("Include numbers? (y/n): ");
    io::stdout().flush().unwrap();
    
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).unwrap();
    let numbers = numbers.trim().to_lowercase() == "y";

    (symbols, numbers)
}


fn get_password_length() -> usize {
    loop {
        print!("Enter desired password length (min 8): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let length: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parse fails, ask again
        };
        
        if length >= 8 {
            return length;
        } else {
            println!("Password length must be at least 8 characters. Try again.");
        }
    }
}

fn generate_password(length: usize, include_symbols: bool, include_numbers: bool) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut pool: Vec<char> = chars.clone();

    if include_symbols {
        pool.extend("!@#$%^&*()_-+=<>,./?;:".chars());
    }
    if include_numbers {
        pool.extend("0123456789".chars());
    }

    let password: String = (0..length)
        .map(|_| pool[rng.gen_range(0..pool.len())])
        .collect();

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


fn main() {
    println!("{}", print_logo());
    
    let length = get_password_length();
    let (symbols, numbers) = get_user_options();
    
    let password = generate_password(length, symbols, numbers);
    
    println!("Generated Password: {}", password);
}
