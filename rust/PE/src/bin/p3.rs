extern crate peutils;
use peutils::primes::sieve_of_eratosthenes;

fn main() {
    untitled3(600851475143);
}

fn untitled3(number: u64) {
    let n_root: u32 = number as u32;
    let n_root = f64::from(n_root).sqrt().round() as usize;
    let primes = sieve_of_eratosthenes(n_root);
    for n in primes {
        if number % n == 0 {
            println!("{}", n);
        }
    }
}