use super::voter::Voter;
use super::{ballot::Ballot, voter::VoterRegistryEntry};
use crate::crypto::elgamal::{self, PublicKey, SecretKey};
use crate::crypto::parameters::CryptoParams;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub private_key: Option<SecretKey>, // Made public for internal service access
    pub encryption_params: CryptoParams,
}

impl Election {
    pub fn new(name: String, choices: Vec<String>) -> Self {
        let encryption_params = CryptoParams::default();
        let (public_key, private_key) = elgamal::generate_keypair(&encryption_params);

        Election {
            id: Uuid::new_v4(),
            name,
            choices,
            voters: HashMap::new(),
            ballots: Vec::new(),
            public_key,
            private_key: Some(private_key),
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

    pub fn nb_ballot(&self) -> usize {
        self.ballots.len()
    }

    pub fn can_tally(&self) -> bool {
        self.private_key.is_some()
    }
}
