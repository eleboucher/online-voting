use crate::Result;
use crate::crypto::elgamal::{Ciphertext, PublicKey, encode_choice, encrypt};
use crate::crypto::parameters::CryptoParams;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub id: Uuid,
    pub ciphertext: Ciphertext,
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

        // Encode the choice as g^index
        let encoded_choice = encode_choice(choice_index, params);

        // Encrypt the encoded choice
        let (ciphertext, _randomness) = encrypt(&encoded_choice, public_key, params);

        Ok(Ballot {
            id: Uuid::new_v4(),
            ciphertext,
        })
    }
    pub fn get_receipt(&self) -> String {
        self.id.to_string()
    }
}
