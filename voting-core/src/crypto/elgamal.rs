use super::parameters::CryptoParams;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicKey {
    pub h: BigUint,
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    pub x: BigUint,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ciphertext {
    pub c1: BigUint,
    pub c2: BigUint,
}

pub fn generate_keypair(params: &CryptoParams) -> (PublicKey, SecretKey) {
    let mut rng = thread_rng();
    let x = rng.gen_biguint_range(&BigUint::one(), &(&params.p - 2u32));
    let h = params.g.modpow(&x, &params.p);
    (PublicKey { h }, SecretKey { x })
}

pub fn encrypt(
    message: &BigUint,
    public_key: &PublicKey,
    params: &CryptoParams,
) -> (Ciphertext, BigUint) {
    let mut rng = thread_rng();
    let r = rng.gen_biguint_range(&BigUint::one(), &(&params.p - 2u32));
    let c1 = params.g.modpow(&r, &params.p);
    let hr = public_key.h.modpow(&r, &params.p);
    let c2 = (message * hr) % &params.p;
    (Ciphertext { c1, c2 }, r)
}

pub fn decrypt(ciphertext: &Ciphertext, secret_key: &SecretKey, params: &CryptoParams) -> BigUint {
    let c1_x = ciphertext.c1.modpow(&secret_key.x, &params.p);
    let c1_x_inv = c1_x.modpow(&(&params.p - 2u32), &params.p);
    (ciphertext.c2.clone() * c1_x_inv) % &params.p
}

/// Homomorphic Addition: E(m1) * E(m2) = E(m1 + m2)
pub fn add_homomorphic(c1: &Ciphertext, c2: &Ciphertext, params: &CryptoParams) -> Ciphertext {
    Ciphertext {
        c1: (c1.c1.clone() * c2.c1.clone()) % &params.p,
        c2: (c1.c2.clone() * c2.c2.clone()) % &params.p,
    }
}

/// Brute force Discrete Log to recover the vote count
/// Solves g^count = target mod p
pub fn solve_discrete_log(
    target: &BigUint,
    params: &CryptoParams,
    max_votes: usize,
) -> Option<usize> {
    let mut temp = BigUint::one(); // g^0
    for i in 0..=max_votes {
        if &temp == target {
            return Some(i);
        }
        temp = (temp * &params.g) % &params.p;
    }
    None
}
