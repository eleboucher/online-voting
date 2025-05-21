use crate::crypto::{commitment::Commitment, zkproof::ZKProof};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub id: Uuid,
    pub commitment: String,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    choice_commitment: Commitment,
}

impl Ballot {
    pub fn new(choice: String) -> Self {
        let commitment_obj = Commitment::new(&choice);
        let commitment = commitment_obj.commitment.clone();
        Ballot {
            id: Uuid::new_v4(),
            commitment,
            choice_commitment: commitment_obj,
        }
    }
    pub fn get_receipt(&self) -> String {
        format!("{}:{}", self.id, self.commitment)
    }
    pub fn generate_proof_for_choice(&self, claimed_choice: &str) -> ZKProof {
        self.choice_commitment
            .create_proof_of_commitment(claimed_choice)
    }

    // Check if this ballot is for a specific choice using the commitment
    pub fn is_vote_for(&self, choice: &str) -> bool {
        let proof = self.generate_proof_for_choice(choice);
        proof.verify()
    }
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_receipt())
    }
}
