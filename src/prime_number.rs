use num::bigint::{BigInt, RandBigInt, BigUint};
use num::bigint::Sign;
use num::{range, pow};
use std::ops::Shr;


pub fn generate_prime_number(bits: u64) -> BigInt {
    // Code below was used to generate the first_primes_list that gets used in ``check_low_level_prime()``:
    let first_primes_list = sieve_of_eratosthenes(8192);
    println!("found {} first primes", first_primes_list.len());


    // Generate ``bits`` large prime number.
    let mut counter = 0;    //DEBUG
    let mut random_bigint = BigInt::default();
    let mut is_prime = false;
    while !is_prime {
        counter += 1;   //DEBUG
        println!("iteration: {}", counter); //DEBUG
        // Generate uneven BigInt ``random_bigint``.
        while random_bigint.clone() % 2 == BigInt::from(0) {
            random_bigint = rand_bigint(bits);
            //println!("DEBUG: big int: {}", random_bigint);  //DEBUG
        }
        //println!("DEBUG: big uneven int: {}", random_bigint);   //DEBUG

        //TODO: fix this shit below. It didn't succeed after 2494833 iterations and seems like it won't ever.
        // Check if ``random_bigint`` passes ``check_low_level_prime()``.
        let result = check_low_level_prime(&random_bigint, &first_primes_list);

        // If ``result`` is true run the miller rabin test.
        if result {
            println!("DEBUG: check_low_level_prime() passed, potential prime: {}", random_bigint); //DEBUG
            let result = is_miller_rabin_passed(&random_bigint);
            println!("miller_rabin_passed: {}", result);
            if result {
                is_prime = true;
            }
        }
    }

    random_bigint
}

/// Generates a random BigInt with the given bit size (``bits``).
fn rand_bigint(bits: u64) -> BigInt {
    let mut number = BigUint::default();

    // generate random numbers until they are exactly the correct bit size.
    // (is it too much, to really check if the number is 100% ``bits`` large? YES... probably)
    let mut rng = rand::thread_rng();
    while BigUint::bits(&number) != bits {
        // generating a 4096 number
        number = rng.gen_biguint(bits);
    }
    // Convert BigUint to BigInt and return it.
    // -> I need to remember that I don't need the ``return`` keyword in rust.
    BigInt::from_biguint(Sign::Plus, number)
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
            return false;
        }
    }
    true
}

/// Runs 20 iterations of the Rabin Miller Primality test over the given ``miller_rabin_candidate``.
fn is_miller_rabin_passed(miller_rabin_candidate: &BigInt) -> bool {
    // number of trials
    let number_of_rabin_trials: u8 = 20;


    let mut max_divisions_by_two: u32 = 0;
    let mut even_component: BigInt = miller_rabin_candidate - BigInt::from(1);

    while &even_component % BigInt::from(2) != BigInt::from(0) {
        even_component = even_component.clone().shr(1);
        max_divisions_by_two += 1;
    }
    //TODO: Keep line number up to date KEKW.
    assert!(
        BigInt::pow(&BigInt::from(2), max_divisions_by_two) * &even_component == miller_rabin_candidate - 1,
        "prime_numbers.rs line 72"
    );

    
    let mut rng = rand::thread_rng();
    for _ in range(0, number_of_rabin_trials) {
        let round_tester = rng.gen_bigint_range(&BigInt::from(2), miller_rabin_candidate);
        // trial composite
        let mut result = true;
        if BigInt::modpow(&round_tester, &even_component, miller_rabin_candidate) == BigInt::from(1) {
            result = false;
        }
        for i in range(0, max_divisions_by_two) {
            if BigInt::modpow(&round_tester, &(&BigInt::from(pow(2, i.try_into().unwrap())) * &even_component), &miller_rabin_candidate) == miller_rabin_candidate - 1 {
                result = false;
            }
        }

        // check trial composite result
        if result {
            return false;
        }
    }
    true
}
