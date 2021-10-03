use std::io;

struct PrimeGenerator {
    current_prime: u64,
}

impl PrimeGenerator {
    fn new() -> PrimeGenerator {
        PrimeGenerator {current_prime: 1}
    }

    fn is_prime(&self) -> bool {
        if self.current_prime % 2 == 0 {
            return true;
        }
        for i in (3..self.current_prime).step_by(2) {
            if self.current_prime % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_prime <= 2 {
            self.current_prime += 1;
        } else {
            loop {
                self.current_prime += 2;
                if self.is_prime() {
                    break;
                }
            }
        }
        Some(self.current_prime)
    }

}

fn main() {
    let mut input_buf = String::new();
    io::stdin().read_line(&mut input_buf).unwrap();
    let mut input: u64 = input_buf.split('\n').next().unwrap().parse().unwrap();
    let mut prime_generator = PrimeGenerator::new();
    let mut current_prime = prime_generator.next().unwrap();
    loop {
        if input % current_prime == 0 {
            print!("{} ", current_prime);
            input /= current_prime;
        } else {
            current_prime = prime_generator.next().unwrap();
        }
        if input == 1 || current_prime as f64 > (input as f64).sqrt() {
            break;
        }
    }
    if input != 1 {
        print!("{} ", input);
    }
}
