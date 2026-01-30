# tk-rand-core

A high-standard random string generator.

## Features
- **ChaCha20 Algorithm**: Uses 20 rounds for conservative, high-security entropy.
- **High Compliant**: Enforces a minimum length of 20 characters.
- **Audit / SBOM Ready**: Built-in methods to report dependency versions at compile time.

## Usage
```rust
use tk_rand_core::generate_nsa_string;

let secret = generate_secure_string(true, true, 32).unwrap();