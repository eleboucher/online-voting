use std::io::{self, Write};
use voting_core::models::{ballot::Ballot, election::Election};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    println!("=== Voting Client CLI (Homomorphic) ===\n");

    let election = Election::new(
        "Remote Election".to_string(),
        vec!["Option A".to_string(), "Option B".to_string()],
    );

    println!("Connected to: {}", election.name);
    println!("Public Key (h): {}\n", election.public_key.h); // Debug print

    println!("Available Choices:");
    for (i, c) in election.choices.iter().enumerate() {
        println!("{}. {}", i, c);
    }

    let input = get_input("\nEnter the index of your choice: ");
    let choice_idx: usize = input.parse().expect("Please enter a number");

    if choice_idx >= election.choices.len() {
        println!("Invalid choice index.");
        return;
    }

    let selected_choice = &election.choices[choice_idx];

    println!("\nGenerating Homomorphic Ballot...");

    let ballot = Ballot::new(
        selected_choice,
        &election.choices,
        &election.public_key,
        &election.encryption_params,
    )
    .expect("Failed to create ballot");

    println!("Ballot Encrypted successfully.");
    println!("Ballot ID: {}", ballot.id);

    println!("\n--- Encrypted Vector Content ---");
    for (i, ciphertext) in ballot.ciphertexts.iter().enumerate() {
        let is_selected = if i == choice_idx { "(Selected)" } else { "" };
        println!("Option {} {}:", i, is_selected);
        println!("  c1: {}", ciphertext.c1);
        println!("  c2: {}", ciphertext.c2);
    }

    println!("\n[Next Step] Send this JSON payload to the Voting Server.");
}
