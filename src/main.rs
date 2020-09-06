use std::io::stdin;
use std::collections::HashMap;

fn main() {
    println!("Welcome to the Sieve of Eratosthenes!");
    println!("Enter the top boundary for calculation: ");

    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            let parsed = buffer.trim_end().parse::<i64>();
            match parsed {
                Ok(limit) => {
                    sieve(limit);
                }
                Err(e) => {
                    println!("Expected a number. Could not read from: {}. {}", buffer, e);
                }
            }
        },
        Err(_) => println!("Unable to read input buffer.")
    }
}

fn sieve(n: i64) {
    let p :i64 = 2;
    println!("Calculating all prime numbers between {} and {}:", p, n);


    let nums: Vec<_> = (p..=n).collect();

    let mut primes: HashMap<i64, bool> = HashMap::new();

    for num in p..=n {
        primes.insert(num, true);
    }

    for num in nums {
        match primes.get(&num) {
            None => println!("Oops!"),
            Some(are_we_prime) => {
                if *are_we_prime {
                    println!("Prime: {}", num);
                    for mynum in 1..=n/num {
                        primes.insert(mynum*num, false);
                    }
                }
            }
        }
    }
}