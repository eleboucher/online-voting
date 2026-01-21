use std::io::{self, Write};
use voting_core::{
    crypto::{
        elgamal::{encode_choice, encrypt},
        parameters::CryptoParams,
    },
    models::{election::Election, voter::Voter},
    services::voting::VotingService,
};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    println!("=== Voting Client CLI ===\n");

    // Simulate fetching election data (in real life, this comes from an API)
    // We create a local dummy just to get the Public Key and Parameters
    let election = Election::new(
        "Remote Election".to_string(),
        vec!["Option A".to_string(), "Option B".to_string()],
    );

    println!("Connected to: {}", election.name);
    println!("Available Choices:");
    for (i, c) in election.choices.iter().enumerate() {
        println!("{}. {}", i, c);
    }

    // Interactive Selection
    let input = get_input("\nEnter the index of your choice: ");
    let choice_idx: usize = input.parse().expect("Please enter a number");

    if choice_idx >= election.choices.len() {
        println!("Invalid choice index.");
        return;
    }

    println!("\nGenerating Encrypted Ballot...");

    // 1. Encode
    let encoded = encode_choice(choice_idx, &election.encryption_params);

    // 2. Encrypt (Client-side encryption using Election Public Key)
    let (ciphertext, _) = encrypt(&encoded, &election.public_key, &election.encryption_params);

    println!("Ballot Encrypted successfully.");
    println!("c1: {}", ciphertext.c1);
    println!("c2: {}", ciphertext.c2);

    println!("\n[Next Step] Send this JSON payload to the Voting Server.");
}
