use super::voter::Voter;
use super::{ballot::Ballot, voter::VoterRegistryEntry};
use crate::crypto::elgamal::{self, decrypt};
use crate::crypto::{elgamal::PublicKey, elgamal::SecretKey, parameters::CryptoParams};
use crate::error::{Result, VotingError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Index;
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Election {
    pub id: Uuid,
    pub name: String,
    pub choices: Vec<String>,
    voters: HashMap<Uuid, VoterRegistryEntry>,
    pub ballots: Vec<Ballot>,
    pub public_key: PublicKey,

    #[serde(skip)]
    private_key: Option<SecretKey>,
    pub encryption_params: CryptoParams,
}

impl Election {
    pub fn new(name: String, choices: Vec<String>) -> Self {
        let encryption_params = CryptoParams::toy_params();
        let (public_key, private_key) = elgamal::generate_keypair(&encryption_params);
        let private_key = Some(private_key);
        Election {
            id: Uuid::new_v4(),
            name,
            choices,
            voters: HashMap::new(),
            ballots: Vec::new(),
            public_key,
            private_key,
            encryption_params,
        }
    }

    pub fn add_voter(&mut self, voter: Voter) {
        self.voters.insert(
            voter.id,
            VoterRegistryEntry {
                voter,
                has_voted: false,
            },
        );
    }

    pub fn nb_ballot(&self) -> usize {
        self.ballots.len()
    }

    pub fn is_allowed_voters(&self, voter_id: Uuid) -> bool {
        self.voters.contains_key(&voter_id)
    }

    pub fn is_valid_choice(&self, choice: &str) -> bool {
        self.choices.contains(&choice.to_string())
    }

    pub fn add_ballot(&mut self, ballot: Ballot) -> Result<String> {
        self.ballots.push(ballot);
        Ok(self.ballots.last().unwrap().get_receipt())
    }
    pub fn has_voted(&self, voter_id: &Uuid) -> bool {
        self.voters
            .get(voter_id)
            .is_some_and(|entry| entry.has_voted)
    }
    pub fn set_voted(&mut self, voter_id: &Uuid) {
        if let Some(entry) = self.voters.get_mut(voter_id) {
            entry.has_voted = true;
        }
    }

    pub fn get_ballot_by_id(&self, ballot_id: Uuid) -> Option<&Ballot> {
        self.ballots.iter().find(|b| b.id == ballot_id)
    }

    pub fn generate_inclusion_proof(&self, receipt: &str) -> Option<String> {
        for (index, ballot) in self.ballots.iter().enumerate() {
            if ballot.id.to_string() == receipt {
                return Some(format!("Ballot found at position {}", index));
            }
        }
        None
    }

    pub fn can_tally(&self) -> bool {
        self.private_key.is_some()
    }
    pub fn get_ballot_choice(&self, ballot: &Ballot) -> Option<&String> {
        let encoded_choice = decrypt(
            &ballot.ciphertext,
            self.private_key.as_ref().unwrap(),
            &self.encryption_params,
        );
        let decrypted_choice =
            elgamal::decode_choice(&encoded_choice, self.choices.len(), &self.encryption_params)?;
        Some(self.choices.index(decrypted_choice))
    }
    pub fn verify_vote(&self, receipt: &str, claimed_choice: &str) -> Option<bool> {
        // Receipt is now just a UUID string, not "uuid:something"
        let ballot_id = Uuid::parse_str(receipt).ok()?;

        // Find the ballot with this ID
        let ballot = self.get_ballot_by_id(ballot_id)?;

        // Decrypt the ballot
        let private_key = self.private_key.as_ref()?;
        let encoded_choice = decrypt(&ballot.ciphertext, private_key, &self.encryption_params);

        // Decode to get choice index
        let decrypted_choice_index =
            elgamal::decode_choice(&encoded_choice, self.choices.len(), &self.encryption_params)?;

        // Check if it matches the claimed choice
        Some(self.choices[decrypted_choice_index] == claimed_choice)
    }
}
