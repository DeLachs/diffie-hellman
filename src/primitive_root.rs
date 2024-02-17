use num::bigint::{BigInt, RandBigInt};
use log::debug;

///TODO: Make a doc string.
pub fn generate_primitive_root(prime: &BigInt) -> BigInt {
    let random_bigint: BigInt;
    random_bigint = rand_uneven_bigint_range(&BigInt::from(1), &(prime - 1));
    /*
    TODO:

    -- assuming that k is a prime and ``prime`` would be a pseudo-prime (p = 2*k+1)
    -- or that k = (``prime``-1)/2

    let random_bigint = 1 -> prime - 1;
    -- random_bigint is a prime if:
    random_bigint != 1 mod p
    random_bigint^2 != 1 mod p
    random_bigint^k != 1 mod p
    */

    random_bigint
}

/// Generates a random ``BigInt`` between ``&lbound`` and ``&ubound``.
fn rand_uneven_bigint_range(lbound: &BigInt, ubound: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    let mut random_bigint = BigInt::default();
    while &random_bigint % 2 == BigInt::from(0) {
        random_bigint = rng.gen_bigint_range(lbound, ubound);
        debug!("random_bigint = {}", random_bigint);
    }
    random_bigint
}
