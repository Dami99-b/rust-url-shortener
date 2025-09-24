# Rust URL Shortener

A tiny in-memory URL shortener built with Rust.  
Made to practice CLI tools, HashMaps, and sync primitives.

## Features
- Generate random short codes (6 chars)
- Store mappings in a HashMap
- Resolve short codes back to URLs
- CLI built with `clap`

## Usage
```bash
cargo run -- shorten "https://example.com"
# Short code: Ab12Xz

cargo run -- resolve Ab12Xz
# Original URL: https://example.com
