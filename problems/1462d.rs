use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut iterable = buffer
        .trim()
        .split("\n")
        .skip(1)
        .map(|line| line.trim());

    while let (Some(_), Some(raw)) = (iterable.next(), iterable.next()) {
        let a: Vec<_> = raw
            .split_whitespace()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();

        let ans = solve(a);
        println!("{}", ans);
    }

    return Ok(());
}

fn solve(a: Vec<u32>) -> usize {
    let n = a.len();
    let s: u32 = a.iter().sum();


    for i in (2..=n).rev() {
        if s % i as u32 != 0 {
            continue;
        }
        let target = s / i as u32;
        let mut aa = a.clone();

        let mut j = 0;
        let mut valid = true;
        let mut ops = 0;
        while j < aa.len() {
            match aa[j].cmp(&target) {
                Ordering::Less => {
                    if j == aa.len() - 1 {
                        valid = false;
                        break;
                    }
                    aa[j] += aa.remove(j + 1);
                    ops += 1;
                        
                },
                Ordering::Equal => j += 1,
                Ordering::Greater => {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            continue;
        }
        return ops;
    }
    return n - 1;
}
