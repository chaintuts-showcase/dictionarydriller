# This file contains a make script for the DictionaryDriller application
#
# Author: Josh McIntyre
#

# This block defines makefile variables
RES_FILES = res/wordlist_sample.txt

BUILD_DIR=target
RES_DIR=target/debug

# This rule builds the application
# Here we are simply wrapping Rust's cargo tool
build: $(RES_FILES)
	cargo build
	cp $(RES_FILES) $(RES_DIR)
	

# This rule cleans the build directory
clean: $(BUILD_DIR)
	rm -rf $(BUILD_DIR)
