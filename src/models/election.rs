use super::ballot::Ballot;
use super::voter::Voter;
use crate::error::{Result, VotingError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Election {
    pub id: Uuid,
    pub name: String,
    pub choices: Vec<String>,
    voters: HashMap<Uuid, Voter>,
    ballots: Vec<Ballot>,

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
        self.voters.insert(voter.id, voter);
    }

    pub fn vote(&mut self, voter_id: Uuid, choice: String) -> Result<String> {
        if !self.voters.contains_key(&voter_id) {
            return Err(VotingError::VoterNotFound);
        }

        if !self.choices.contains(&choice) {
            return Err(VotingError::InvalidChoice);
        }

        let ballot = Ballot::new(choice);
        let receipt = ballot.clone().get_receipt();
        self.ballots.push(ballot);
        Ok(receipt)
    }

    pub fn nb_ballot(&self) -> usize {
        self.ballots.len()
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

    pub fn tally_votes(&self) -> HashMap<String, usize> {
        let mut res: HashMap<String, usize> = HashMap::new();

        for choice in &self.choices {
            res.insert(choice.clone(), 0);
        }

        for ballot in &self.ballots {
            for choice in &self.choices {
                if ballot.is_vote_for(choice) {
                    *res.get_mut(choice).unwrap() += 1;
                }
            }
        }
        res
    }
}
