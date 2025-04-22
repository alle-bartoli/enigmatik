
mod utils;

use utils::{print_logo, get_password_length, get_user_options, generate_password};
use std::io::{self, Write};

fn main() {
    loop {
        println!("{}", print_logo());
        println!("Press 'q' or 'c' to quit.");

        // Ask for password length
        println!("\nStep 1: Configure Password");
        let length = get_password_length();

        // Ask for user options
        println!("\nStep 2: Choose Options");
        let (include_symbols, include_numbers) = get_user_options();

        // Generate the password
        println!("\nStep 3: Generating Password...");
        let password = generate_password(length, include_symbols, include_numbers);
        println!("Generated Password: {}\n", password);

        // Prompt the user for next action
        print!("Generate another password or quit? (Press 'q' or 'c' to quit): ");
        io::stdout().flush().unwrap();

        let mut action = String::new();
        io::stdin().read_line(&mut action).unwrap();

        if action.trim().eq_ignore_ascii_case("q") || action.trim().eq_ignore_ascii_case("c") {
            println!("Exiting program. Goodbye!");
            break;
        }

        println!("\nStarting over...\n");
    }
}
