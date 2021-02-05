// main.rs by https://github.com/kaubu
// An example of how to use the module

use std::io::{self, Write};

mod password;

fn main() {
	let option = get_input("[Panes Password Tester]\nWhat option would you like to use?\n[1] Hash a password\n[2] Check a hash and a password\n>> ");
	let option: i32 = option.parse().unwrap_or(-1);

	match option {
		-1 => {
			println!("Not a number!");
		}
		1 => { // Hash a password
			let pass = get_input("Password: ");
			println!("Hash: {:?}", password::hash_password(pass)); // Get's hash and prints it instantly
		},
		2 => { // Check a hash and a password
			let hash = get_input("Hash: ");
			let pass = get_input("Password: ");

			let pass_check = password::is_password(hash, pass); // Checks the hash and the password

			match pass_check {
				true => {
					println!("Password is correct.");
				},
				false => {
					println!("Password is incorrect.");
				}
			}
		},
		_ => {
			println!("Not an option!");
		}
	}

	get_input("Press ENTER to exit.");
}

fn get_input(message: &str) -> String {
	let mut input = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	
	io::stdin().read_line(&mut input).expect("Failed to read line.");

	input.trim().to_owned()
}