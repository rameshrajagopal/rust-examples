use std::env;

fn loop_method(n: i32) {
    let mut count = n.clone();
    loop {
        if count % 2 == 0 {
            count = count / 2;
        } else {
            count = count * 2;
        }
        println!("{}", count);
        if count == n {
            break;
        }
    }
}

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
    loop_method(big_n);

    let mut numbers: Vec<u32> = Vec::new();
    for i in 0..10 {
        let x = i * i;
        numbers.push(x);
    }
    for num in numbers {
        println!("{}", num);
    }
}
