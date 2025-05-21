use super::commitment::verify_commitment;

pub struct ZKProof {
    pub commitment: String,
    pub claimed_choice: String,
    pub nonce: String,
}

impl ZKProof {
    pub fn verify(&self) -> bool {
        verify_commitment(&self.commitment, &self.claimed_choice, &self.nonce)
    }
}
