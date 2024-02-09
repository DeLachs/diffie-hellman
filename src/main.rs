mod benchmark;
mod prime_number;

use num::bigint::{BigInt, RandBigInt};
use num::pow::pow;
use rand::Rng;
use prime_number::generate_prime_number;

//? The simple thing is working correct.
//? I am just not good enough to get the real one working.

fn simple_example() {
    //? simple example
    // both exchange the numbers below
    let n: u64 = 59; // n needs to be 4096 long and a Primzahl
    const G: u64 = 34; // small prime number

    // A keeps this number personal
    const GEHEIM_A: u32 = 9; // between 1 and N
                             // B keeps this number personal
    const GEHEIM_B: u32 = 11; // between 1 and N

    // A sends the number below to B
    let public_ag = G.pow(GEHEIM_A) % &n;
    // B gets public_ag and uses to generate key.
    let private_bag = public_ag.pow(GEHEIM_B) % &n;
    println!("Key Output: {}", private_bag);

    //// B sends the number below to A
    //let public_bg = G.pow(GEHEIM_B) % &n;
    //// A gets public_bg and uses to generate key.
    //let private_abg = public_bg.pow(GEHEIM_A) % &n;
    //println!("Key Output: {}", private_abg);
}

fn main() {
    //the_real_thing();
    //simple_example();
    time_function!(generate_prime_number(4096));
}

//TODO unfinished
fn the_real_thing() {
    // both numbers get exchanged between the two.
    let n = rand_bigint(4096); // 4096 long number (soll ne primzahl sein???)
    let g = rand_bigint(128); // small prime number

    // generating secrets for A and B
    let mut rng = rand::thread_rng();
    let lower_bound = 1;
    //let upper_bound: BigInt = BigInt::from(usize::MAX);
    let upper_bound = usize::MAX;

    println!("{}", u32::MAX);

    //? these numbers should be between 1 and n but due to limitations and my low power brain it will
    //? be between 1 and usize.
    // A keeps this number personal
    let secret_number_a = rng.gen_range(lower_bound..upper_bound);
    println!("Secret number a: {}", secret_number_a);
    // B keeps this number personal
    let secret_number_b = rng.gen_range(lower_bound..upper_bound);
    println!("Secret number b: {}", secret_number_b);

    // A will send the following number to B.
    let public_ag = pow(g, secret_number_a) % &n;
    println!("The public ag number from A is: {}", public_ag);
    // B gets public_ag from A and generates the private key.
    //let private_bag = pow(g, secret_number_b) % &n;
}

fn rand_bigint(bits: u64) -> BigInt {
    let mut number = BigInt::default();

    // generate random numbers until they are exactly the correct bit size.
    // (it this too much? YES... probably)
    while BigInt::bits(&number) != bits {
        // generating a 4096 number
        let mut rng = rand::thread_rng();
        number = rng.gen_bigint(bits);
    }
    // print big number and size for verification.
    println!("The number is: {}", number);
    println!("The size of the number is: {}bit", BigInt::bits(&number));
    // return number -> I need to remember that this is how return works in rust.
    number
}
