mod crypto;
mod models;
use models::election::Election;
use models::voter::Voter;
mod error;

fn main() {
    let voter1 = Voter::new();
    let voter2 = Voter::new();
    let mut election = Election::new(
        "Sample Election".to_string(),
        vec!["Option A".to_string(), "Option B".to_string()],
    );
    election.add_voter(voter1.clone());
    election.add_voter(voter2.clone());

    let receipt = election.vote(voter1.id, "Option A".to_string()).unwrap();
    let receipt2 = election.vote(voter2.id, "Option B".to_string()).unwrap();
    println!("Receipt 1: {}", receipt);
    println!("Receipt 2: {}", receipt2);
    println!("{}", election.nb_ballot());
    let vote_res = election.tally_votes();
    println!("{:?}", vote_res);

    let is_valid = election.verify_vote("some_receipt", "Option A");
    println!("Vote verification: {}", is_valid);
    let is_valid = election.verify_vote(&receipt, "Option A");
    println!("Vote verification: {}", is_valid);

    let inclusion_proof = election.generate_inclusion_proof(&receipt);
    println!("Inclusion proof: {:?}", inclusion_proof);
}
