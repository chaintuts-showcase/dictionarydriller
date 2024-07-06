/* This file contains code that conducts a dictionary attack on a password hash
*
* Author: Josh McIntyre
*/
use argon2;
use base64;
use base64::Engine;
use std::env;
use std::io::{Error, ErrorKind};

mod dictionarydriller 
{
	use argon2;
	use argon2::PasswordVerifier;
	
	/* This function conducts the dictionary attack */
	pub fn attack(argon2_hash: argon2::PasswordHash, wordlist_filename: &String) -> (bool, String)
	{
		let mut found = false;
		let mut found_password = String::from("");
		for word_line in std::fs::read_to_string(wordlist_filename).unwrap().lines()
		{
			// Get the word from file
			let word = word_line.to_string();
			
			// Use the argon2 libraries verify_password functionality to check
			// the word against the supplied password hash
			// Print information about the match if found
			let hash_match = argon2::Argon2::default().verify_password(word.as_bytes(), &argon2_hash).is_ok();
			if hash_match
			{
				found_password = word;
				found = true;
			}		
		}
		
		return (found, found_password);
	}
}

/* The main entry point for the program */
fn main() -> Result<(), Error>
{	
	// Get command line arguments
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 3
	{
		println!("Usage ./dictionarydriller [wordlist_filename] [password_hash]");
		return Err(Error::new(ErrorKind::Other, "wordlist_filename, password_hash required"));
	}
	let wordlist_filename: &String = &args[1];
	let password_hash: &String = &args[2];
	
	// Create an argon2 PasswordHash object from the string encoded hash
	// Extract the salt for later use
	let argon2_hash = argon2::PasswordHash::new(&password_hash).unwrap();
	let salt_b64 = argon2_hash.salt.unwrap().as_str();
	let salt_bytes = base64::engine::general_purpose::STANDARD.decode(salt_b64).unwrap();
	let salt = std::str::from_utf8(&salt_bytes).unwrap();
	
	// Print out base info
	println!("Hash: {password_hash}");
	println!("Salt: {salt}");
	
	// Try the dictionary attack
	println!("Running attack");
	let(found, found_password) = dictionarydriller::attack(argon2_hash, wordlist_filename);

	// Report the results
	println!("Attack finished");
	if found == true
	{
		println!("Password found: {found_password}");
	}
	else
	{
		println!("No password found for the specified hash");
	}

	return Ok(());
}

#[cfg(test)]
mod tests;
