use std::collections::HashMap;

use crate::crypto::commitment::Commitment;
use crate::error::{Result, VotingError};
use crate::models::ballot::Ballot;
use crate::models::election::Election;
use uuid::Uuid;
pub struct VotingService;

impl VotingService {
    pub fn vote(election: &mut Election, voter_id: Uuid, choice: &str) -> Result<String> {
        if !election.is_allowed_voters(voter_id) {
            return Err(VotingError::VoterNotFound);
        }

        if !election.is_valid_choice(choice) {
            return Err(VotingError::InvalidChoice);
        }
        if election.has_voted(&voter_id) {
            return Err(VotingError::AlreadyVoted);
        }
        // Create a new commitment for the choice
        let commitment = Commitment::new(choice);
        let ballot = Ballot::new(commitment);
        let receipt = election.add_ballot(ballot)?;
        election.set_voted(&voter_id);
        Ok(receipt)
    }

    pub fn verify_vote(election: &Election, receipt: &str, claimed_choice: &str) -> bool {
        let parts: Vec<&str> = receipt.split(':').collect();
        if parts.len() != 2 {
            return false;
        }
        let ballot_id = Uuid::parse_str(parts[0]).unwrap();

        if let Some(ballot) = election.get_ballot_by_id(ballot_id) {
            // Verify the commitment
            let proof = ballot.generate_proof_for_choice(claimed_choice);
            if proof.verify() {
                return true;
            }
        }
        false
    }

    pub fn tally(election: &Election) -> HashMap<String, usize> {
        let mut tally = HashMap::new();
        for ballot in &election.ballots {
            let choice = ballot.commitment.clone();
            *tally.entry(choice).or_insert(0) += 1;
        }
        tally
    }
}
