use std::collections::BTreeSet;
use std::io::{self, BufRead};

#[derive(Debug)]
struct PrimeChecker {
    found_primes: BTreeSet<u32>,
}

impl PrimeChecker {
    fn new(threshold: u32) -> Self {
        let mut found_primes = BTreeSet::new();
        found_primes.insert(2);
        for i in (3..threshold).step_by(2) {
            let mut is_prime = true;
            for prime in found_primes.iter() {
                if prime * prime > i {
                    break;
                }
                if i % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                found_primes.insert(i);
            }
        }
        PrimeChecker { found_primes }
    }

    fn check(&self, candidate: u32) -> bool {
        self.found_primes.contains(&candidate)
    }
}

fn depth_first_search(
    odds: &Vec<&u32>,
    evens: &Vec<&u32>,
    odd_id: usize,
    traversed_evens: &mut Vec<bool>,
    matching: &mut Vec<i32>,
    prime_checker: &PrimeChecker,
) -> bool {
    for i in 0..evens.len() {
        if prime_checker.check(odds[odd_id] + evens[i]) && !traversed_evens[i] {
            traversed_evens[i] = true;
            if matching[i] == -1
                || depth_first_search(
                    odds,
                    evens,
                    matching[i] as usize,
                    traversed_evens,
                    matching,
                    prime_checker,
                )
            {
                matching[i] = odd_id as i32;
                return true;
            }
        }
    }
    false
}

fn main() {
    let prime_checker = PrimeChecker::new(60000);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let _: u32 = line.unwrap().parse().unwrap();
        let input_nums: Vec<u32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let input_odds: Vec<&u32> = input_nums.iter().filter(|&x| *x & 1 == 1).collect();
        let input_evens: Vec<&u32> = input_nums.iter().filter(|&x| *x & 1 == 0).collect();

        let mut matching = vec![-1; input_evens.len()];
        let mut traversed_evens = vec![false; input_evens.len()];
        let mut count = 0;
        for i in 0..input_odds.len() {
            traversed_evens.iter_mut().map(|x| *x = false).count();
            if depth_first_search(
                &input_odds,
                &input_evens,
                i,
                &mut traversed_evens,
                &mut matching,
                &prime_checker,
            ) {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
