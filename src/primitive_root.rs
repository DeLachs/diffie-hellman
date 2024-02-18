use num::bigint::{BigInt, RandBigInt};
use num::Integer;
use log::debug;

///TODO: Make a doc string.
pub fn generate_primitive_root(prime: &BigInt) -> BigInt {
    /*
    source: https://math.stackexchange.com/questions/2190342/simple-question-about-generating-random-primitive-roots-for-a-large-prime
    -- assuming that k is a prime and ``prime`` would be a pseudo-prime (p = 2*k+1)
    -- or that k = (``prime``-1)/2  // Probably wrong... but it is enough

    -- This will be fine...
    let random_bigint = 1 -> prime - 1;
    -- random_bigint is a prime if:
    random_bigint != 1 mod p
    random_bigint^2 != 1 mod p
    random_bigint^k != 1 mod p
    */

    let lbound = BigInt::from(1);
    let ubound = prime - 1;
    let mut random_bigint = rand_uneven_bigint_range(&lbound, &ubound);
    let one_mod_p = BigInt::from(1).mod_floor(&prime);
    let mut counter = 1;
    // let k = (prime - BigInt::from(1)) / BigInt::from(2);
    // Won't use the third check, because it doesn't work and would be waaaaay to in-performant && (&random_bigint.pow(k) == &one_mod_p)
    while (&random_bigint == &one_mod_p) || (&random_bigint.pow(2) == &one_mod_p) {
        counter += 1;
        random_bigint = rand_uneven_bigint_range(&BigInt::from(1), &(prime - 1));
    }
    debug!("Iterations {}", counter);
    debug!("Pseudo primitive root: {}", random_bigint);
    random_bigint
}

/// Generates a random ``BigInt`` between ``&lbound`` and ``&ubound``.
fn rand_uneven_bigint_range(lbound: &BigInt, ubound: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    let mut random_bigint = BigInt::default();
    while &random_bigint % 2 == BigInt::from(0) {
        random_bigint = rng.gen_bigint_range(lbound, ubound);
    }
    random_bigint
}

/*
I stopped doing it the right way because I wanted some progress and wanted to do something other.
Sources for future me:

- https://en.wikibooks.org/wiki/Algorithm_Implementation/Mathematics/Modular_Exponentiation#python
- https://cp-algorithms.com/algebra/primitive-root.html
- https://math.stackexchange.com/questions/124408/finding-a-primitive-root-of-a-prime-number
- https://en.wikipedia.org/wiki/Euler%27s_totient_function
- https://math.stackexchange.com/questions/1446305/how-to-find-primitive-roots-of-big-numbers-modulo-n-like-121

fn generator(prime: &BigInt) -> BigInt {
    let mut fact: Vec<BigInt> = Vec::new();
    let mut phi = prime - BigInt::from(1);
    let mut n = phi;

    let mut i = BigInt::from(2);
    while &i * &i <= n {
        i += BigInt::from(1);
        if n.mod_floor(&i) == BigInt::from(0) {
            fact.push(i.clone());
            while n.mod_floor(&i) == BigInt::from(0) {
                n /= &i;
            }
        }
    }
    if n > BigInt::from(1) {
        fact.push(n);
    }

    let mut res = BigInt::from(2);
    while &res <= p {
        res += 1;
        let mut ok = true;

        let mut i = BigInt::from(0);
        while &i < &BigInt::from(fact.len()) && ok {
            i += 1;
            ok &= powmod(&res, &(phi / fact[i]) &prime) != 1;
        }
        if ok {
            return res;
        }
    }
    BigInt::from(-1)
}

fn powmod(a: &BigInt, b: &BigInt, p: &BigInt) -> BigInt {
    let mut accum = BigInt::from(1);
    let mut apow2 = a.clone();
    while b > &BigInt::from(0) {
        if b.bitand(BigInt::from(1)) {
            accum = (accum * apow2) % p;
        }
        apow2 = (apow2 * apow2) % p;
        b = b >> 1
    }

    BigInt::from(1)
}
*/