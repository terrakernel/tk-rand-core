# tk-rand-core

A high-standard random string generator.

## Features
- **ChaCha20 Algorithm**: Uses 20 rounds for conservative, high-security entropy.
- **High Compliant**: Enforces a minimum length of 20 characters.
- **Audit / SBOM Ready**: Built-in methods to report dependency versions at compile time.


## Usage
```rust
use tk_rand_core::{generate_nsa_string, get_audit_metadata};

/// Generate a secure random string with a minimum length of 32 characters.
let secret = generate_secure_string(true, true, 32).unwrap();

/// Get the audit metadata for the current binary.
let metadata = get_audit_metadata();
println!("--- SBOM AUDIT REPORT ---");
println!("Component: {}", metadata.crate_name);
println!("Version:   {}", metadata.crate_version);
println!("Security Dependencies:");
for dep in metadata.security_dependencies {
    println!("  - {}: {}", dep.name, dep.version);
}

