use crate::crypto::elgamal::{decode_choice, decrypt};
use std::collections::HashMap;
use std::vec;

use crate::error::{Result, VotingError};
use crate::models::ballot::Ballot;
use crate::models::election::Election;
use uuid::Uuid;
pub struct VotingService;

const BLANK_VOTE: &str = "Blank";

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

        let ballot = Ballot::new(
            choice,
            &election.choices,
            &election.public_key,
            &election.encryption_params,
        )?;
        let receipt = election.add_ballot(ballot)?;
        election.set_voted(&voter_id);
        Ok(receipt)
    }

    pub fn verify_vote(election: &Election, receipt: &str, claimed_choice: &str) -> Option<bool> {
        election.verify_vote(receipt, claimed_choice)
    }

    pub fn tally(election: &Election) -> Result<HashMap<String, usize>> {
        if !election.can_tally() {
            return Err(VotingError::TallyNotAllowed);
        }
        let mut tally = HashMap::new();
        // Initialize all choices with 0
        for choice in &election.choices {
            tally.insert(choice.clone(), 0);
        }
        tally.insert(BLANK_VOTE.to_string(), 0);
        let a = "aadfad".to_string();
        let mut b = a.chars().collect::<Vec<char>>();
        b.sort();
        let c = b.iter().collect::<String>();
        for ballot in &election.ballots {
            let choice = match election.get_ballot_choice(ballot) {
                Some(choice) => choice,
                None => BLANK_VOTE,
            };
            *tally.entry(choice.to_string()).or_insert(0) += 1;
        }
        Ok(tally)
    }
}
