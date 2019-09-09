use std::collections::HashSet;

pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes = vec![];
    let mut crossed_out: HashSet<u32> = HashSet::new();

    for i in 2..n {
        if crossed_out.contains(&i) {
            continue;
        } else {
            primes.push(i);
            let mut k = i;
            while i * k < n {
                crossed_out.insert(i * k);
                k += 1;
            }
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    // provided tests
    #[test]
    fn test_sieve_basic() {
        assert_eq!(vec![2, 3, 5, 7, 11], sieve(12));
    }

    // my own tests
    #[test]
    fn test_n_is_a_prime() {
        assert_eq!(vec![2, 3, 5, 7], sieve(11));
    }
}