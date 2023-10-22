use std::cmp;
use std::fmt::Display;
use std::io::{self, BufRead};
use std::iter::zip;
use std::ops;

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

type Digit = u8;

struct DigitResult {
    new_digit: Digit,
    new_carry: Digit,
}

fn add_digits(x: Digit, y: Digit, carry: Digit) -> DigitResult {
    let z = x + y + carry;
    let (new_digit, new_carry) = (z % 10, z / 10);

    DigitResult {
        new_digit,
        new_carry,
    }
}

fn mul_digits(x: Digit, y: Digit, carry: Digit) -> DigitResult {
    let z = x * y + carry;
    let (new_digit, new_carry) = (z % 10, z / 10);

    DigitResult {
        new_digit,
        new_carry,
    }
}

// Digits are in ascending order from 10^0
#[derive(Clone, Debug)]
struct BigUInt {
    digits: Vec<Digit>,
}

impl BigUInt {
    fn new(n: u32) -> Self {
        BigUInt {
            digits: to_digit_vec(n),
        }
    }
}

impl Display for BigUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.digits
            .iter()
            .rev()
            .skip_while(|&&x| x == 0)
            .map(|x| write!(f, "{}", x))
            .collect::<std::fmt::Result>()?;
        Ok(())
    }
}

impl ops::Add<BigUInt> for BigUInt {
    type Output = BigUInt;

    fn add(self, rhs: BigUInt) -> BigUInt {
        let lhs_len = self.digits.len();
        let rhs_len = rhs.digits.len();
        let mut result = Vec::with_capacity(cmp::max(lhs_len, rhs_len) + 1);

        let (xs, ys) = pad_with(self.digits, rhs.digits, 0);
        let final_carry = zip(xs, ys).fold(0, |carry, (x, y)| {
            let digit_res = add_digits(x, y, carry);
            result.push(digit_res.new_digit);
            digit_res.new_carry
        });

        result.push(final_carry as u8);
        BigUInt { digits: result }
    }
}

impl ops::Mul<BigUInt> for BigUInt {
    type Output = BigUInt;

    fn mul(self, rhs: BigUInt) -> BigUInt {
        let lhs_len = self.digits.len();
        let rhs_len = rhs.digits.len();

        let mut steps = Vec::with_capacity(cmp::max(lhs_len, rhs_len));

        // for each digit in the multiplier
        // multiply with each digit in the multiplicant
        // creating a new digit and carrying any overflow
        let mut i = 0;
        for x in self.digits {
            let mut step_digits = Vec::with_capacity(lhs_len + rhs_len);
            step_digits.append(&mut vec![0; i]);

            let final_carry = rhs.digits.iter().fold(0, |carry, y| {
                let DigitResult {
                    new_digit,
                    new_carry,
                } = mul_digits(x, *y, carry);
                step_digits.push(new_digit);
                new_carry
            });

            step_digits.push(final_carry);
            steps.push(BigUInt {
                digits: step_digits,
            });

            i += 1;
        }

        // sum steps
        if let Some(result) = steps.into_iter().reduce(|acc, n| acc + n) {
            result
        } else {
            BigUInt { digits: vec![0] }
        }
    }
}

// Takes two vectors and pads the shorter with the given element
// before returning both equal length vectors.
fn pad_with<A: Clone>(mut xs: Vec<A>, mut ys: Vec<A>, pad: A) -> (Vec<A>, Vec<A>) {
    let x_len = xs.len();
    let y_len = ys.len();

    if x_len == y_len {
        return (xs, ys);
    }

    let pad_size = x_len.abs_diff(y_len);
    let mut padding = vec![pad; pad_size];

    if x_len < y_len {
        xs.append(&mut padding);
    } else {
        ys.append(&mut padding);
    }

    (xs, ys)
}

// Converts an integer into a vector of digits (ascending)
fn to_digit_vec(n: u32) -> Vec<Digit> {
    n.to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn factorial(n: u32) -> BigUInt {
    if n == 0 {
        return BigUInt::new(1);
    }

    BigUInt::new(n) * factorial(n - 1)
}

fn extraLongFactorials(n: i32) {
    if n < 0 {
        println!("Error: Negative factorial requested");
    } else {
        let res = factorial(n as u32);
        println!("Result: {}", res);
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
