use num::BigUint;
use num::{bigint::ToBigUint, traits::One};
use std::io::{self, BufRead};

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn factorial(n: u32) -> BigUint {
    if n <= 0 {
        return One::one();
    }

    let mut results: Vec<BigUint> = Vec::with_capacity(n as usize);
    results.insert(0, One::one());
    results.insert(1, One::one());

    for i in 2..n {
        let i_big = i.to_biguint().unwrap();
        let previous;
        unsafe {
            previous = results.get_unchecked(i as usize - 1);
        };
        results.insert(i as usize, i_big * previous);
    }

    let previous;
    unsafe {
        previous = results.get_unchecked(n as usize - 1);
    }

    n.to_biguint().unwrap() * previous
}

fn extraLongFactorials(n: i32) {
    if n < 0 {
        println!("Error: Negative factorial requested");
    } else {
        let res = factorial(n as u32);
        println!("{}", res);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    extraLongFactorials(n);
}
