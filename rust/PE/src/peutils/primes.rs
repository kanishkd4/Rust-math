pub fn sieve_of_eratosthenes(n: usize) -> Vec<u64> {
    let n_total: usize = n;
    let mut n_vec = vec![1; n_total];
    n_vec[0] = 0;
    n_vec[1] = 0;
    let n = n_total as f64;
    let n_root = n.sqrt().round() as usize;
    let n = n as usize;

    for i in 2..=n_root {
        if n_vec[i] == 1 {
            let j_iter = i.pow(2)..n;
            for j in j_iter.into_iter().step_by(i) {
                n_vec[j] = 0;
            }
        }
    }
    let mut primes = Vec::new();
    let upper_limit = n as u64;
    for i in 0..upper_limit {
        if n_vec[i as usize] == 1 {
            primes.push(i);
        }
    }
    // println!("Primes up to {}: {:?}", n, primes.len());
    return primes;
}

