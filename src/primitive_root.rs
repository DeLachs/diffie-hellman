use num::bigint::{BigInt, RandBigInt};
use num::{range, Integer};
use std::ops::Shr;
use std::process::exit;
use log::debug;

pub fn generate_primitive_root(prime: &BigInt) -> BigInt {
    /*
    TODO:

    -- assuming that k is a prime and prime would be a pseudo-prime (p = 2*k+1)
    -- or that k = (prime-1)/2

    let random_bigint = 1 -> prime - 1;
    -- random_bigint is a prime if:
    random_bigint != 1 mod p
    random_bigint^2 != 1 mod p
    random_bigint^k != 1 mod p
    */
    prime.clone()
}
