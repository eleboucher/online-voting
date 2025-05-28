use num_bigint::BigUint;
use voting_core::crypto::{
    elgamal::{decode_choice, decrypt, encode_choice, encrypt, generate_keypair},
    parameters::CryptoParams,
};

#[test]
fn test_basic_encryption() {
    let params = CryptoParams::toy_params();
    let (public_key, secret_key) = generate_keypair(&params);

    // Test encrypting and decrypting a message
    let message = BigUint::from(7u32);
    let (ciphertext, _) = encrypt(&message, &public_key, &params);
    let decrypted = decrypt(&ciphertext, &secret_key, &params);

    assert_eq!(message, decrypted);
}

#[test]
fn test_vote_encryption() {
    let params = CryptoParams::toy_params();
    let (public_key, secret_key) = generate_keypair(&params);

    // Simulate voting for choice 1 (out of 3 choices)
    let choice_index = 1;
    let num_choices = 3;

    // Encode and encrypt the choice
    let encoded = encode_choice(choice_index, &params);
    let (ciphertext, _) = encrypt(&encoded, &public_key, &params);

    // Decrypt and decode
    let decrypted = decrypt(&ciphertext, &secret_key, &params);
    let decoded_choice = decode_choice(&decrypted, num_choices, &params);

    assert_eq!(decoded_choice, Some(choice_index));
}
