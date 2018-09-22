extern crate peutils;
use peutils::primes::sieve_of_eratosthenes;

fn main() {
    let primes = sieve_of_eratosthenes(1000_000);
    println!("{:?}", primes[10000]);
}

// I can use the sieve and just count but there will be a more elegant solution