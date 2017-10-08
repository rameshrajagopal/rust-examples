use std::env;

fn main() {
    let arg = env::args().nth(1).map(|s| s.to_owned()).unwrap_or_else(|| "5".to_owned());
    println!("value n {}", &arg);
    let n : i32  = arg.parse().unwrap();

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("n is zero");
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!("n is a small increase to 10 fold");
            10 * n
        } else {
            println!("n is a big number, reduce by 2");
            n/2
        };
    println!("{} -> {}", n, big_n);
}
