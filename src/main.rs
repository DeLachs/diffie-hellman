mod benchmark;
mod prime_number;
mod primitive_root;

use prime_number::generate_prime_number;
use primitive_root::generate_primitive_root;

use num::bigint::{BigInt, RandBigInt};

fn main() {
    // Initialize logging with env_logger
    env_logger::init();

    // 4096 long number needs to be a prime number.
    let p = time_function!(generate_prime_number(4096));
    let g = time_function!(generate_primitive_root(&p));

    // Initialize person A and person B with p and g
    let mut a = Person::new(&p, &g);
    let mut b = Person::new(&p, &g);

    // let a and b generate a personal secret number
    a.generate_secret();
    b.generate_secret();

    // let a and b calculate gsp_to_send
    a.calculate_gsp();
    b.calculate_gsp();

    // exchange gsp
    a.gsp_received = b.gsp_to_send.clone();
    b.gsp_received = a.gsp_to_send.clone();

    // calculate key
    a.calculate_key();
    b.calculate_key();

    // check equality of keys
    assert_eq!(a.key, b.key);

    println!("{}", a.key);
    println!("{}", b.key);
}

struct Person {
    p: BigInt,
    g: BigInt,
    secret_number: BigInt,
    gsp_to_send: BigInt,
    gsp_received: BigInt,
    key: BigInt
}

impl Person {
    fn new(p: &BigInt, g: &BigInt) -> Person {
        Person {
            p: p.clone(),
            g: g.clone(),
            secret_number: BigInt::default(),
            gsp_to_send: BigInt::default(),
            gsp_received: BigInt::default(),
            key: BigInt::default()
        }
    }

    fn generate_secret(&mut self) {
        let mut rng = rand::thread_rng();
        self.secret_number = rng.gen_bigint_range(&BigInt::from(2), &(self.p.clone() - BigInt::from(2)))
    }

    fn calculate_gsp(&mut self) {
        self.gsp_to_send = self.g.modpow(&self.secret_number, &self.p);
    }

    fn calculate_key(&mut self) {
        self.key = self.gsp_received.modpow(&self.secret_number, &self.p);
    }
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
