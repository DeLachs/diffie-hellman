use num::bigint::{BigInt, RandBigInt, BigUint};
use num::bigint::Sign;


pub fn generate_prime_number(bits: u64) -> BigInt {
    // Code below was used to generate the first_primes_list that gets used in ``check_low_level_prime()``:
    let first_primes_list = sieve_of_eratosthenes(8192);
    //println!("found {} first primes", first_primes_list.len());

    // Generate BigInt
    let random_bigint = rand_bigint(bits);

    // check if ``random_bigint`` passes ``check_low_level_prime()``
    let result = check_low_level_prime(&random_bigint, &first_primes_list);
    println!("{}", result);


    return BigInt::from(8);
}

/// Generates a random BigInt with the given bit size (``bits``).
fn rand_bigint(bits: u64) -> BigInt {
    let mut number = BigUint::default();

    // generate random numbers until they are exactly the correct bit size.
    // (is it too much, to really check if the number is 100% ``bits`` large? YES... probably)
    while BigUint::bits(&number) != bits {
        // generating a 4096 number
        let mut rng = rand::thread_rng();
        number = rng.gen_biguint(bits);
    }
    // Convert BigUint to BigInt and return it.
    // -> I need to remember that I don't need the ``return`` keyword in rust.
    BigInt::from_biguint(Sign::NoSign, number)
}

/// Generates a list of prime numbers smaller then n.
fn sieve_of_eratosthenes(n: usize) -> Vec<BigInt> {
    let mut prime_vec = vec![true; n + 1];
    let mut p = 2;
    while p * p <= n {
        // If prime[n] is not changed, then it is a prime.
        if prime_vec[p] == true {
            // update all multiples of p
            for i in (p * p..n + 1).step_by(p) {
                prime_vec[i] = false;
            }
        }
        p += 1;
    }
    
    let mut result_prime_vec = Vec::new();
    for p in 2..n + 1 {
        if prime_vec[p] {
            result_prime_vec.push(BigInt::from(p));
        }
    }
    
    result_prime_vec
}

/// Checks with low level prime numbers (with the first few hundred) if the ``candidate`` is a prime number.
fn check_low_level_prime(candidate: &BigInt, first_primes_list: &Vec<BigInt>) -> bool {
    for divisor in first_primes_list.iter() {
        if (candidate % divisor == BigInt::from(0)) && (&BigInt::pow(divisor, 2) <= candidate) {
            return false
        }
    }
    true
}