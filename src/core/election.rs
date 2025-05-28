use super::voter::Voter;
use super::{ballot::Ballot, voter::VoterRegistryEntry};
use crate::error::{Result, VotingError};
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

    encryption_key: String,
}

impl Election {
    pub fn new(name: String, choices: Vec<String>) -> Self {
        Election {
            id: Uuid::new_v4(),
            name,
            choices,
            voters: HashMap::new(),
            ballots: Vec::new(),
            encryption_key: Uuid::new_v4().to_string(),
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
        let parts = receipt.split(':').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return None;
        }
        let ballot_id = parts[0];
        for (index, ballot) in self.ballots.iter().enumerate() {
            if ballot.id.to_string() == ballot_id {
                return Some(format!("Ballot found at position {}", index));
            }
        }
        None
    }
    pub fn verify_vote(&self, receipt: &str, claimed_choice: &str) -> bool {
        let parts = receipt.split(':').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return false;
        }
        let ballot_id = parts[0];
        for ballot in &self.ballots {
            if ballot.id.to_string() == ballot_id {
                let proof = ballot.generate_proof_for_choice(claimed_choice);
                return proof.verify();
            }
        }
        false
    }
}
