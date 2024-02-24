use num::bigint::{BigInt, RandBigInt};

/// calculates the public number ``gsp_to_send`` = ``g`` to the power of ``secret`` mod ``p``
pub fn calculate_gsp(g: &BigInt, secret: &BigInt, p: &BigInt) -> BigInt {
    g.modpow(secret, p)
}

/// Calculates the private key ``key`` = ``gsp_received`` to the power of ``secret`` mod ``p``
pub fn calculate_key(gsp_received: &BigInt, secret: &BigInt, p: &BigInt) -> BigInt {
    gsp_received.modpow(secret, p)
}

/// Generate random secret for use in the diffie hellman key exchange
pub fn generate_secret(p: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    rng.gen_bigint_range(&BigInt::from(2), &(p.clone() - BigInt::from(2)))
}
