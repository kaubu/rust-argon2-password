use std::io::{self, Write};
use argon2::{self, Config, Variant, ThreadMode};
use ring::rand::{self, SecureRandom};
use data_encoding::HEXUPPER;

const SALT_LENGTH: usize = 32;

fn main() {
	let password = input("Password: ");
	let hash = hash_password(password);

	println!("Hash Result: {:?}\n[Password Checker]", hash);

	let hash = input("Hash: ");
	let password = input("Password: ");

	let password_match = is_password(hash, password);

	match password_match {
		true => { println!("Password does match."); },
		false => { println!("Password does not match."); }
	}
}

fn generate_salt() -> String {
	let rng = rand::SystemRandom::new();
	let mut salt = [0u8; SALT_LENGTH];

	rng.fill(&mut salt).unwrap();

	HEXUPPER.encode(&salt)
}

fn hash_password(password: String) -> String {
	let password = password.as_bytes();
	let default_config = Config::default();

	let config = Config {
		variant: Variant::Argon2id,
		version: default_config.version,
		mem_cost: default_config.mem_cost,
		time_cost: 4, // Amount of passes
		lanes: 4, // Amount of cores
		thread_mode: ThreadMode::Parallel,
		secret: default_config.secret,
		ad: default_config.ad,
		hash_length: 32
	};

	let salt = generate_salt();

	argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap() // Returns the hash
}

fn is_password(hash: String, password: String) -> bool {
	let password = password.as_bytes();

	argon2::verify_encoded(&hash, password).expect("[ERROR] Invalid hash or other error.")
}

fn input(message: &str) -> String {
	let mut input = String::new();

	print!("{}", &message);
	io::stdout().flush().unwrap();
	
	io::stdin().read_line(&mut input).expect("Failed to read line.");

	input.trim().to_owned()
}