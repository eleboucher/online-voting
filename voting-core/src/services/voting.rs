use crate::crypto::elgamal::{self, Ciphertext};
use crate::error::{Result, VotingError};
use crate::models::ballot::Ballot;
use crate::models::election::Election;
use num_bigint::BigUint;
use num_traits::One;
use std::collections::HashMap;
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

    pub fn tally(election: &Election) -> Result<HashMap<String, usize>> {
        if !election.can_tally() {
            return Err(VotingError::TallyNotAllowed);
        }

        let num_candidates = election.choices.len();
        if election.ballots.is_empty() {
            return Ok(HashMap::new());
        }

        // 1. Initialize encrypted totals with (1, 1) representing Encrypt(0)
        let mut encrypted_totals: Vec<Ciphertext> = (0..num_candidates)
            .map(|_| Ciphertext {
                c1: BigUint::one(),
                c2: BigUint::one(),
            })
            .collect();

        // 2. Homomorphic Aggregation
        // The server sums the ciphertexts without knowing what's inside
        for ballot in &election.ballots {
            for (i, c) in ballot.ciphertexts.iter().enumerate() {
                if i < encrypted_totals.len() {
                    encrypted_totals[i] = elgamal::add_homomorphic(
                        &encrypted_totals[i],
                        c,
                        &election.encryption_params,
                    );
                }
            }
        }

        // 3. Decrypt and Discrete Log
        let mut results = HashMap::new();
        let private_key = election.private_key.as_ref().unwrap();

        for (i, encrypted_total) in encrypted_totals.iter().enumerate() {
            // This decrypts to g^count
            let m_g_pow_count =
                elgamal::decrypt(encrypted_total, private_key, &election.encryption_params);

            // Find 'count' such that g^count = decrypted_val
            let count = elgamal::solve_discrete_log(
                &m_g_pow_count,
                &election.encryption_params,
                election.ballots.len() + 1,
            )
            .unwrap_or(0);

            results.insert(election.choices[i].clone(), count);
        }

        Ok(results)
    }
}
