/// Calculate Pi to the nth decimal place.
/// $$ \pi = \sum_{k=0}^\infty \left[   \frac{1}{16^k} \! \left(   \frac{4}{8k+1} - \frac{2}{8k+4} - \frac{1}{8k+5} - \frac{1}{8k+6}   \right) \right] $$
use num_bigint::{BigUint, ToBigUint};
use std::io::Write;
// fn prompt_input(prompt: &str) -> u64 {
//     println!("{prompt}");

//     let mut input = String::new();
//     let _ = std::io::stdin().read_line(&mut input);
//     let n: u64 = input.trim().parse().expect("请输入一个正整数");
//     n
// }

/// We First Init a BigUint 10^(n+2) to prepare for
/// the / operation. if the next term to add is smaller
/// than 1, we can just break, and get the answer.
///
/// Output: BigUint, n+2 digits.
///         should add a "." in second position.
fn calculate_pi(n: u64) -> BigUint {
    let mut each_poch: BigUint = BigUint::from(10u32).pow(n as u32 + 8);
    let mut sum: BigUint = BigUint::ZERO;
    let mut k: u32 = 0;

    while each_poch != BigUint::ZERO {
        let temp = do_frac_part(each_poch.clone(), k);
        sum += temp.clone();
        // println!("{:-013}", temp);

        if temp < 1u32.to_biguint().unwrap() {
            break;
        }

        k += 1;
        each_poch /= 16u32;
    }
    sum / 100_000_000u32
}

fn do_frac_part(main_part: BigUint, k: u32) -> BigUint {
    main_part.clone() * 4u32 / (8u32 * k + 1)
        - main_part.clone() * 2u32 / (8u32 * k + 4)
        - main_part.clone() / (8u32 * k + 5)
        - main_part.clone() / (8u32 * k + 6)
}

fn main() {
    // let n = prompt_input("Please input the number of digits: ");
    let n = 100000_u64;

    // output to file
    let mut file = std::fs::File::create("pi.txt").expect("create failed");
    let pi = calculate_pi(n);
    file.write_all(format!("3.{}", &pi.to_string()[1..n as usize - 1]).as_bytes())
        .expect("write failed");
    // println!("{:?}", calculate_pi(n));
}
