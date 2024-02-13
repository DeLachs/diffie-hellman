mod benchmark;
mod prime_number;
mod primitive_root;

use prime_number::generate_prime_number;
use primitive_root::generate_primitive_root;

fn main() {
    // Initialize logging with env_logger
    env_logger::init();

    // 4096 long number needs to be a prime number.
    let p = time_function!(generate_prime_number(4096));
    //println!("p: {}", p);
    //TODO: let g = primitive root of n
    let g = time_function!(generate_primitive_root(&p));
    println!("g: {}", g);
    //TODO: let secret_number_a = secret_number_a >= 1 && secret_number_a <= n - 2
}

//fn the_real_thing() {
//    // both numbers get exchanged between the two.
//    let n = generate_prime_number(4096); // 4096 long number needs to be a prime number.
//    let g = generate_prime_number(128); // small prime number
//
//    // generating secrets for A and B
//    let mut rng = rand::thread_rng();
//    let lower_bound = 1;
//    //let upper_bound: BigInt = BigInt::from(usize::MAX);
//    let upper_bound = usize::MAX;
//
//    //? these numbers should be between 1 and n but due to limitations and my low power brain it will
//    //? be between 1 and usize.
//    // A keeps this number personal
//    let secret_number_a = rng.gen_range(lower_bound..upper_bound);
//    println!("Secret number a: {}", secret_number_a);
//    // B keeps this number personal
//    let secret_number_b = rng.gen_range(lower_bound..upper_bound);
//    println!("Secret number b: {}", secret_number_b);
//
//    // A will send the following number to B.
//    let public_ag = pow(g, secret_number_a) % &n;
//    println!("The public ag number from A is: {}", public_ag);
//    // B gets public_ag from A and generates the private key.
//    //let private_bag = pow(g, secret_number_b) % &n;
//}
