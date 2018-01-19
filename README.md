# sam-rs
Rust compiler plugin for compile time x64 assembling.

## DEPRECATED
This release is really buggy and it isn't possible to realize it like this. This project is now deprecated, for a working and stable version go to the new version of sam: https://github.com/ioncodes/sam

## Usage
```rust
#![feature(plugin)]
#![plugin(sam)]

fn main() {
	let asm = sam!("mov eax, 3"); // returns a string with the bytes
	println!("{}", asm);
}
```

## Building
1. Get the keystone library and headers.
2. Compile ks/main.cpp
3. ```cargo build```
4. Put ```ks``` wherever your plugin is.
