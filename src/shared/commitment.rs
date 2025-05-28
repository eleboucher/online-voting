use super::zkproof::ZKProof;
use blake3::Hasher;
use hex;
use rand::TryRngCore;
use rand::rngs::OsRng;

#[derive(Debug, Clone, Default)]
pub struct Commitment {
    pub commitment: String,
    choice: String,
    nonce: String,
}

impl Commitment {
    pub fn new(choice: &str) -> Self {
        let mut nonce_bytes = [0u8; 32];
        OsRng.try_fill_bytes(&mut nonce_bytes).unwrap();
        let nonce = hex::encode(nonce_bytes);
        let commitment = compute_commitment(choice, &nonce);
        Commitment {
            commitment,
            choice: choice.to_string(),
            nonce,
        }
    }

    pub fn create_proof_of_commitment(&self, claimed_choice: &str) -> ZKProof {
        if claimed_choice == self.choice {
            ZKProof {
                commitment: self.commitment.clone(),
                claimed_choice: claimed_choice.to_string(),
                nonce: self.nonce.clone(),
            }
        } else {
            ZKProof {
                commitment: self.commitment.clone(),
                claimed_choice: self.choice.clone(),
                nonce: "invalid".to_string(),
            }
        }
    }
}

fn compute_commitment(choice: &str, nonce: &str) -> String {
    let mut hash = Hasher::new();
    hash.update(choice.as_bytes());
    hash.update(nonce.as_bytes());
    let result = hash.finalize();
    hex::encode(result.as_bytes())
}

pub fn verify_commitment(commitment: &str, claimed_choice: &str, nonce: &str) -> bool {
    let computed_commitment = compute_commitment(claimed_choice, nonce);
    commitment == computed_commitment
}
