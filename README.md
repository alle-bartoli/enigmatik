# Enigmatik 🔐

A secure and interactive terminal-based password generator built with Rust.
Generate strong, customizable passwords with ease while enjoying a stylish interface.

## Features

- **Interactive Input**: Choose your desired password length, with a minimum of 8 characters.
- **Customization**: Optionally include symbols and/or numbers in your password.
- **Animated Generation**: Watch as your password is generated with a fun animation.
- **Stylish Logo**: A visually appealing ASCII art logo greets you on startup.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or later)

### Clone the Repository

```bash
git clone https://github.com/alle-bartoli/enigmatik.git
cd enigmatik
```

## Build and Run

### Build the project

```sh
cargo build --release
```

### Run the app

```sh
cargo run
```

## Usage

1. Start the application and follow the prompts:
   • Enter your desired password length (minimum 8 characters).
   • Choose whether to include symbols and/or numbers. 2. The application will generate a strong password for you and display it.

2. The application will generate a strong password for you and display it.

```sh
Enter desired password length (min 8): 12
Include symbols? (y/n): y
Include numbers? (y/n): y
Generating...
Generated Password: aB$7gH@9pZ2!
```

## Dependencies

• `rand`: A Rust library for generating random values.

## Contributing

Contributions are welcome!
If you have ideas for improvements or new features, feel free to open an issue or submit a pull request.

### Steps to Contribute

1. Fork the repository.
2. Create a new branch (git checkout -b feature-branch).
3. Commit your changes (git commit -m "Add a new feature").
4. Push to the branch (git push origin feature-branch).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

Thank you for using `Enigmatik`! Stay secure. 🔐
