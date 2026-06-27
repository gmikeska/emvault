//! Framework-agnostic configuration + hex helpers shared by the consuming apps.
//!
//! These utilities are not Bitcoin-specific (so they don't belong in
//! `emvault-core`) but were duplicated byte-for-byte across every app's
//! `config.rs`. They live here, always available (no backend feature needed),
//! so each app loads its own `AppConfig` from the environment using one shared
//! set of primitives instead of re-implementing them.

/// Configuration loading errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// A required environment variable was not set.
    #[error("required env var `{var}` is not set")]
    Missing {
        /// The variable name.
        var: &'static str,
    },
    /// A variable was set but failed to parse.
    #[error("env var `{var}` is invalid: {reason}")]
    Parse {
        /// The variable name.
        var: &'static str,
        /// Human-readable reason.
        reason: String,
    },
}

/// Read a required environment variable.
///
/// # Errors
/// Returns [`ConfigError::Missing`] if `var` is not present in the environment.
pub fn require(var: &'static str) -> Result<String, ConfigError> {
    std::env::var(var).map_err(|_| ConfigError::Missing { var })
}

/// Read an optional environment variable, treating empty strings as absent.
#[must_use]
pub fn optional(var: &'static str) -> Option<String> {
    std::env::var(var).ok().filter(|s| !s.is_empty())
}

/// Decode a hex string into bytes.
///
/// # Errors
/// Returns a human-readable error string if `s` has an odd length or contains a
/// non-hex character.
pub fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd-length hex string ({} chars)", s.len()));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|e| format!("invalid hex at byte {}: {e}", i / 2))
        })
        .collect()
}

/// Encode bytes as a lowercase hex string.
#[must_use]
pub fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(out, "{b:02x}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{hex_decode, hex_encode};

    #[test]
    fn hex_round_trip() {
        let bytes = [0x00u8, 0x0f, 0xa5, 0xff];
        assert_eq!(hex_encode(&bytes), "000fa5ff");
        assert_eq!(hex_decode("000fa5ff").unwrap(), bytes);
    }

    #[test]
    fn hex_encode_empty() {
        assert_eq!(hex_encode(&[]), "");
        assert_eq!(hex_decode("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn hex_decode_rejects_odd_length() {
        let err = hex_decode("abc").unwrap_err();
        assert!(err.contains("odd-length"), "got: {err}");
    }

    #[test]
    fn hex_decode_rejects_non_hex() {
        let err = hex_decode("zz").unwrap_err();
        assert!(err.contains("invalid hex"), "got: {err}");
    }
}
