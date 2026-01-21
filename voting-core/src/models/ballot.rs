use crate::Result;
use crate::crypto::elgamal::{Ciphertext, PublicKey, encrypt};
use crate::crypto::parameters::CryptoParams;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub id: Uuid,
    // Vector of ciphertexts: [Enc(0), Enc(1), Enc(0)]
    pub ciphertexts: Vec<Ciphertext>,
}

impl Ballot {
    pub fn new(
        choice: &str,
        choices: &[String],
        public_key: &PublicKey,
        params: &CryptoParams,
    ) -> Result<Self> {
        let choice_index = choices
            .iter()
            .position(|c| c == choice)
            .ok_or(crate::VotingError::InvalidChoice)?;

        let mut ciphertexts = Vec::new();

        for i in 0..choices.len() {
            // Encrypt 1 if selected, 0 otherwise
            // Note: We encrypt g^1 or g^0 because ElGamal is multiplicative
            let m_pow = if i == choice_index { 1u32 } else { 0u32 };
            let m = params.g.modpow(&BigUint::from(m_pow), &params.p);

            let (c, _) = encrypt(&m, public_key, params);
            ciphertexts.push(c);
        }

        Ok(Ballot {
            id: Uuid::new_v4(),
            ciphertexts,
        })
    }

    pub fn get_receipt(&self) -> String {
        self.id.to_string()
    }
}
