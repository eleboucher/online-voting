use super::parameters::CryptoParams;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicKey {
    /// h = g^x mod p (where x is the secret key)
    pub h: BigUint,
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    /// The secret exponent
    pub x: BigUint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ciphertext {
    /// c1 = g^r mod p
    pub c1: BigUint,
    /// c2 = m * h^r mod p
    pub c2: BigUint,
}

pub fn generate_keypair(params: &CryptoParams) -> (PublicKey, SecretKey) {
    let mut rng = thread_rng();

    // Generate random secret key x in range [1, p-2]
    let x = rng.gen_biguint_range(&BigUint::one(), &(&params.p - 2u32));

    // Compute public key h = g^x mod p
    let h = params.g.modpow(&x, &params.p);

    (PublicKey { h }, SecretKey { x })
}

/// Encrypt a message (for votes, this will be g^vote_choice)
pub fn encrypt(
    message: &BigUint,
    public_key: &PublicKey,
    params: &CryptoParams,
) -> (Ciphertext, BigUint) {
    let mut rng = thread_rng();

    // Generate random r in range [1, p-2]
    let r = rng.gen_biguint_range(&BigUint::one(), &(&params.p - 2u32));

    // c1 = g^r mod p
    let c1 = params.g.modpow(&r, &params.p);

    // c2 = m * h^r mod p
    let hr = public_key.h.modpow(&r, &params.p);
    let c2 = (message * hr) % &params.p;

    (Ciphertext { c1, c2 }, r)
}

/// Decrypt a ciphertext
pub fn decrypt(ciphertext: &Ciphertext, secret_key: &SecretKey, params: &CryptoParams) -> BigUint {
    // Compute c1^x mod p
    let c1_x = ciphertext.c1.modpow(&secret_key.x, &params.p);

    // Compute the inverse of c1^x mod p
    let c1_x_inv = c1_x.modpow(&(&params.p - 2u32), &params.p);

    // m = c2 * (c1^x)^(-1) mod p
    (ciphertext.c2.clone() * c1_x_inv) % &params.p
}

/// Encode a vote choice as a group element
/// We use exponential ElGamal: encode choice i as g^i
pub fn encode_choice(choice_index: usize, params: &CryptoParams) -> BigUint {
    params.g.modpow(&BigUint::from(choice_index), &params.p)
}

/// Decode a group element back to a choice (by trying all possibilities)
pub fn decode_choice(
    encoded: &BigUint,
    num_choices: usize,
    params: &CryptoParams,
) -> Option<usize> {
    for i in 0..num_choices {
        let test = encode_choice(i, params);
        if test == *encoded {
            return Some(i);
        }
    }
    None
}
