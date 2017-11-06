# sam-rs
Rust compiler plugin for compile time x64 assembling.

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
