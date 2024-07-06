/* This file contains code for unit testing the DictionaryDriller functionality
*
* Author: Josh McIntyre
*/
use crate::dictionarydriller;

/* Test the dictionary attack with a known hash and password from the sample list */
#[test]
fn test_attack()
{
	let password_hash = String::from("$argon2i$v=19$m=16,t=2,p=1$Zm9vYmFyYmF6$YMoljsb76DqkMZExF1AZZg");
	let wordlist_filename = String::from("res/wordlist_sample.txt");
	let argon2_hash = argon2::PasswordHash::new(&password_hash).unwrap();

	let (found, found_password) = dictionarydriller::attack(argon2_hash, &wordlist_filename);
	
	assert_eq!(found, true);
	assert_eq!(found_password, String::from("rockyou"));
}