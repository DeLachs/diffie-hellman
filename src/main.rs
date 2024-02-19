mod benchmark;
mod prime_number;
mod primitive_root;
mod client;
mod server;

use std::process::exit;
use std::env;
use log::{info, error};
use env_logger::Env;
use num::bigint::{BigInt, RandBigInt};

use prime_number::generate_prime_number;
use primitive_root::generate_primitive_root;
use client::send_stream;
use server::server;

struct Args {
    is_server: bool,
    addr: String,
    passwd: String,
}

fn main() {
    // Initialize logging with env_logger and the default level of info.
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    //TODO: something for later
    /*
    if args.len() == 4 && (args[2] == "-c".to_string() || args[2] == "-c".to_string()) {
        println!("yay");
    }
    */
    println!("{:?}", args);

    let args = Args {
        is_server: if args[1] == "-s".to_string() {true} else {false},
        addr: args[2].clone(),
        passwd: args[3].clone(),
    };

    //TODO: the sending and receiving
    if args.is_server {
        let result = server(args.addr.as_str());
        let result = match result {
            Ok(()) => true,
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };
        info!("{}", result);
    } else {
        let result = time_function!(send_stream(args.addr.as_str(), args.passwd.as_str()));
        let result = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };
        info!("msg: {}", result);
    }


    /*
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
    */
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



//? Notes:
/*
let mut stream = TcpStream::connect("127.0.0.1:34612")?;

is equal to:

let mut stream = TcpStream::connect("127.0.0.1:34612")?;
let mut stream = match stream {
    Ok(c) => c,
    Err(e) => return Err(e)
};
*/
