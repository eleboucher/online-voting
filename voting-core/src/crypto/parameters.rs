//! Cryptographic parameters for the voting system

use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

/// Cryptographic parameters for the group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoParams {
    pub p: BigUint,
    pub g: BigUint,
}

impl CryptoParams {
    pub fn toy_params() -> Self {
        CryptoParams {
            p: BigUint::from(23u32), // not secure, but good for testing
            g: BigUint::from(4u32),
        }
    }

    /// Create slightly larger test parameters
    pub fn test_params() -> Self {
        // Still not secure, but better for testing
        CryptoParams {
            p: BigUint::from(1019u32),
            g: BigUint::from(2u32),
        }
    }
}
