use voting_core::{
    models::{election::Election, voter::Voter},
    services::voting::VotingService,
};

#[test]
fn test_basic_election_flow() {
    // Create election
    let mut election = Election::new(
        "Test Election".to_string(),
        vec!["Yes".to_string(), "No".to_string()],
    );

    // Add voters
    let voter1 = Voter::new();
    let voter2 = Voter::new();
    let voter3 = Voter::new();
    election.add_voter(voter1.clone());
    election.add_voter(voter2.clone());
    election.add_voter(voter3.clone());

    // Cast votes
    VotingService::vote(&mut election, voter1.id, "Yes").unwrap();
    VotingService::vote(&mut election, voter2.id, "Yes").unwrap();
    VotingService::vote(&mut election, voter3.id, "No").unwrap();

    // Tally
    let results = VotingService::tally(&election).unwrap();

    // Check results
    assert_eq!(results.get("Yes"), Some(&2));
    assert_eq!(results.get("No"), Some(&1));
}

#[test]
fn test_double_voting_prevented() {
    let mut election = Election::new("Test".to_string(), vec!["A".to_string(), "B".to_string()]);

    let voter = Voter::new();
    election.add_voter(voter.clone());

    // First vote should succeed
    assert!(VotingService::vote(&mut election, voter.id, "A").is_ok());

    // Second vote should fail
    assert!(VotingService::vote(&mut election, voter.id, "B").is_err());
}
