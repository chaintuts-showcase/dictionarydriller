## General
____________

### Author
* Josh McIntyre

### Website
* jmcintyre.net

### Overview
* DictionaryDriller is a simple dictionary attack password cracking tool

## Development
________________

### Git Workflow
* master for releases (merge development)
* development for bugfixes and new features

### Building
* make build
Build the application - wraps `cargo build`
* make clean
Clean the build directory

### Features
* Parse the Argon2 cryptographic hash of a password
* Read words from a specified wordlist filename
* Verify the hash for each attempted word and report if a matching password is found

### Requirements
* Requires Rust development tools

### Platforms
* Windows
* MacOSX
* Linux

## Usage
____________

### Command Line Usage
* Run `./dictionarydriller <wordlist_filename> <password_hash>` to crack the Argon2 hash using the specified wordlist_filename
* Example: "$argon2i$v=19$m=16,t=2,p=1$Zm9vYmFyYmF6$YMoljsb76DqkMZExF1AZZg" - password "rockyou" with salt "foobarbaz"
