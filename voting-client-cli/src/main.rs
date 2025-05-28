use voting_core::{
    crypto::{
        elgamal::{encode_choice, encrypt},
        parameters::CryptoParams,
    },
    models::{ballot::Ballot, election::Election, voter::Voter},
    services::voting::VotingService,
};

fn main() {
    println!("=== Simple Voting Client ===\n");
    let voter = Voter::new();
    // Step 1: Create a mock election to get its public key
    let mut election = Election::new(
        "Test Election".to_string(),
        vec!["Option A".to_string(), "Option B".to_string()],
    );
    election.add_voter(voter.clone());
    println!("Election: {}", election.name);
    println!("Choices: {:?}", election.choices);
    println!("Public Key: {:?}\n", election.public_key.h);

    // Step 2: User selects a choice (hardcoded for now)
    let my_choice = "Option B";
    let choice_index = election
        .choices
        .iter()
        .position(|c| c == my_choice)
        .expect("Invalid choice selected");
    // Step 3: Encrypt the choice
    // Using the election's public key and crypto parameters

    let encrypted_choice = encode_choice(choice_index, &election.encryption_params);
    let encrypted_ballot = encrypt(
        &encrypted_choice,
        &election.public_key,
        &election.encryption_params,
    );

    println!("Encrypted Ballot: {:?}", encrypted_ballot);
    // Step 4: Submit the ballot
    // In a real application, this would be sent to the election server
    let receipt =
        VotingService::vote(&mut election, voter.id, my_choice).expect("Failed to submit ballot");
    println!("Ballot submitted successfully!");
    println!("Receipt: {}", receipt);
    // Step 5: Verify the vote
    // In a real application, this would be done by the election server
    let is_valid = VotingService::verify_vote(&election, &receipt, my_choice);
    println!(
        "Vote verification for choice '{}': {}",
        my_choice,
        is_valid.unwrap_or(false)
    );

    // Step 6: Generate inclusion proof
    // In a real application, this would be done by the election server
    // This is a mock proof for demonstration purposes
    let inclusion_proof = election.generate_inclusion_proof(&receipt);
    println!(
        "Inclusion proof for receipt '{}': {:?}",
        receipt, inclusion_proof
    );

    // Step 7: Tally the votes
    let tally_result = VotingService::tally(&election);
    match tally_result {
        Ok(tally) => {
            println!("Vote Tally:");
            for (choice, count) in tally {
                println!("{}: {}", choice, count);
            }
        }
        Err(e) => println!("Error during tallying: {}", e),
    }
}
