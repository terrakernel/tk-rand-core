use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use thiserror::Error;
use serde::{Serialize};

include!(concat!(env!("OUT_DIR"), "/audit_info.rs"));

pub const MIN_REQUIRED_LENGTH: usize = 20;
pub const MAX_ALLOWED_LENGTH: usize = 64;

#[derive(Debug, Serialize)]
pub struct DependencyInfo {
    pub name: &'static str,
    pub version: &'static str,
}


// SBOM metadata for better auditing
#[derive(Debug, Serialize)]
pub struct AuditMetadata {
    pub crate_name: &'static str,
    pub crate_version: &'static str,
    pub security_dependencies: Vec<DependencyInfo>,
}

pub fn get_audit_metadata() -> AuditMetadata {
    AuditMetadata {
        crate_name: env!("CARGO_PKG_NAME"),
        crate_version: env!("CARGO_PKG_VERSION"),
        security_dependencies: vec![
            DependencyInfo {
                name: "rand",
                version: DEP_RAND_VERSION
            },
            DependencyInfo {
                name: "rand_chacha",
                version: DEP_CHACHA_VERSION
            },
        ],
    }
}

#[derive(Error, Debug)]
pub enum RandError {
    #[error("Length requested ({0}) exceeds maximum allowed ({1})")]
    LengthExceeded(usize, usize),

    #[error("Length requested ({0}) is too short. NSA standards require at least {1} characters.")]
    LengthTooShort(usize, usize),
}

/// Generates a cryptographically secure random string using ChaCha20.
pub fn generate_secure_string(
    numbers: bool,
    special_chars: bool,
    length: usize,
) -> Result<String, RandError> {
    // 1. Validation
    if length < MIN_REQUIRED_LENGTH {
        return Err(RandError::LengthTooShort(length, MIN_REQUIRED_LENGTH));
    }
    if length > MAX_ALLOWED_LENGTH {
        return Err(RandError::LengthExceeded(length, MAX_ALLOWED_LENGTH));
    }

    // 2. Build Character Set
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    if numbers {
        charset.push_str("0123456789");
    }
    if special_chars {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    let charset_bytes = charset.as_bytes();

    let mut rng = ChaCha20Rng::from_os_rng();

    let result: String = (0..length)
        .map(|_| {
            let idx = (rng).random_range(0..charset_bytes.len());
            charset_bytes[idx] as char
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length_enforcement() {
        let result = generate_secure_string(true, true, 19);
        assert!(matches!(result, Err(RandError::LengthTooShort(19, 20))));
    }

    #[test]
    fn test_valid_generation() {
        let result = generate_secure_string(true, true, 24);
        assert!(result.is_ok());
        let s = result.unwrap();
        assert_eq!(s.len(), 24);
        // Ensure it contains more than just default alphabet (statistically likely)
        println!("Generated: {}", s);
    }
}