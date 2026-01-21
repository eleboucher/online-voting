use voting_core::models::election::Election;
use voting_core::models::voter::Voter;
use voting_core::services::voting::VotingService;

fn main() {
    println!("--- Electronic Voting Server (ElGamal) ---");

    // 1. Setup Election
    let mut election = Election::new(
        "Presidential Election 2026".to_string(),
        vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
        ],
    );

    // 2. Register mock voters
    let voters: Vec<Voter> = (0..5).map(|_| Voter::new()).collect();
    for v in &voters {
        election.add_voter(v.clone());
    }

    println!("Election '{}' initialized.", election.name);
    println!("Candidates: {:?}", election.choices);
    println!("Registered {} voters.\n", voters.len());

    // 3. Simulate Voting
    // In a real app, this would be an API endpoint
    let votes_to_cast = vec![
        (0, "Alice"),
        (1, "Bob"),
        (2, "Alice"),
        (3, "Charlie"),
        (0, "Bob"), // Double vote attempt
    ];

    println!("--- Processing Incoming Votes ---");
    for (voter_idx, choice) in votes_to_cast {
        let voter = &voters[voter_idx];
        print!("Voter {} voting for {}: ", voter.id, choice);

        match VotingService::vote(&mut election, voter.id, choice) {
            Ok(receipt) => println!("Success! Receipt: {}", receipt),
            Err(e) => println!("Failed: {}", e),
        }
    }

    // 4. Tally
    println!("\n--- Election Tally ---");
    match VotingService::tally(&election) {
        Ok(results) => {
            for (candidate, count) in results {
                println!("{}: {} votes", candidate, count);
            }
        }
        Err(e) => println!("Tally failed: {}", e),
    }
}
