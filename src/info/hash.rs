#[cfg(feature = "hashing")]
use sha2::{Digest, Sha256};

/// Get reproducible hash for grouping of same variables
pub fn get_hash(var_type: &str, name: &str, package_name: &str) -> Option<String> {
    #[cfg(feature = "hashing")]
    {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}:{}", var_type, name, package_name));
        Some(format!("{:x}", hasher.finalize()))
    }

    #[cfg(not(feature = "hashing"))]
    {
        let _ = (var_type, name, package_name);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash_none_without_feature() {
        #[cfg(not(feature = "hashing"))]
        assert!(get_hash("i32", "var", "pkg").is_none()); // You never know for sure...
    }

    #[test]
    #[cfg(feature = "hashing")]
    fn test_get_hash_consistency() {
        // Testing the hashing lib itself might be redundant, but I like having tests.
        let hash1 = get_hash("i32", "var", "pkg");
        let hash2 = get_hash("i32", "var", "pkg");
        assert_eq!(hash1, hash2);
        assert!(hash1.is_some());
    }
}
