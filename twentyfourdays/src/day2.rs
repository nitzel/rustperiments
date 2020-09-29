extern crate num;
extern crate primal;
use primal::Sieve;

/// Total number of divisors calculated
/// by multiplying the exponents (increased by 1) of the factors
fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
  match primes.factor(n) {
    Ok(factors) => {
      Some(factors.into_iter()
                  .fold(1, 
                    |acc, (_, x)| acc * (x + 1)))
    }
    Err(_) => None
  }
}

pub fn main() {
    println!("Hello, world! ");
    let sieve = Sieve::new(10000);
    let suspect = 5273;
    println!("{} is a prime: {}", suspect, sieve.is_prime(suspect));
    let not_a_prime = 1024;
    println!("{} is a prime: {}", not_a_prime, sieve.is_prime(not_a_prime));
    
    let n = 1000;
    match sieve.primes_from(0).nth(n - 1) {
      Some(number) => println!("{}th prime is {}", n, number),
      None => println!("I don't know anything about the {}nth prime", n)
    }
    println!("{:?}", sieve.factor(2610));
    println!("{:?}", num_divisors(2610, &sieve));
    
}
