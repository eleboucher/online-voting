use voting_core::models::election::Election;
use voting_core::models::voter::Voter;
use voting_core::services::voting::VotingService;

fn main() {
    let voter1 = Voter::new();
    let voter2 = Voter::new();
    let mut election = Election::new(
        "Sample Election".to_string(),
        vec!["Option A".to_string(), "Option B".to_string()],
    );
    election.add_voter(voter1.clone());
    election.add_voter(voter2.clone());
    let receipt = VotingService::vote(&mut election, voter1.id, "Option B").unwrap();
    let receipt2 = VotingService::vote(&mut election, voter2.id, "Option A").unwrap();
    println!("Receipt 1: {}", receipt);
    println!("Receipt 2: {}", receipt2);
    println!("{}", election.nb_ballot());
    let vote_res = VotingService::tally(&election);
    println!("{:?}", vote_res);

    let is_valid = election
        .verify_vote("some_receipt", "Option A")
        .unwrap_or(false);
    println!("Vote verification: {}", is_valid);
    let is_valid = election.verify_vote(&receipt, "Option A").unwrap_or(false);
    println!("Vote verification: {}", is_valid);

    let inclusion_proof = election.generate_inclusion_proof(&receipt);
    println!("Inclusion proof: {:?}", inclusion_proof);
}
