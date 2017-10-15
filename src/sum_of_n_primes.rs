use std::env;

fn  is_prime(num: &i32) -> bool {
    let mid = (num / 2) + 1;
    for idx in 2..mid {
        if num % idx == 0 {
            return false
        }
    }
    true
}

fn sum_of_nprimes(n: i32) -> u64 {
    let mut count = 0i32;
    let mut sum   = 0u64;

    for idx in 2.. {
        if is_prime(&idx) {
            count += 1;
            sum += idx as u64;
        }

        if count == n {
            break;
        }
    }
    sum
}

fn main() {
    if let Some(number) = env::args().nth(1) {
        println!("{}", sum_of_nprimes(number.parse::<i32>().unwrap()));
    } else {
        println!("Invalid Usage\n ./program <number>");
    }
}
