use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

/// Cryptographic parameters for the group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoParams {
    pub p: BigUint,
    pub g: BigUint,
}

impl CryptoParams {
    pub fn default() -> Self {
        CryptoParams {
            p: BigUint::from(1019u32),
            g: BigUint::from(2u32),
        }
    }
}
