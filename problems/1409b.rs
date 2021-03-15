use std::io::prelude::*;
use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut iterable = buffer
        .trim()
        .split('\n')
        .skip(1);

    while let Some(line) = iterable.next() {
        let input: Vec<_> = line
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        let (a, b, x, y, n) = (input[0], input[1], input[2], input[3], input[4]);

        let ans = solve(a, b, x, y, n);
        println!("{}", ans);
    }

    return Ok(());
}

fn solve(a: u64, b: u64, x: u64, y: u64, n: u64) -> u64 {
    let at1 = cmp::min(a - x,  n);
    let bt1 = cmp::min(b - y,  n);

    let at2 = cmp::min(b - y, n - at1);
    let bt2 = cmp::min(a - x, n - bt1);

    let aa = (a - at1) * (b - at2);
    let bb = (b - bt1) * (a - bt2);
    
    if aa < bb { 
        return aa;
    } else {
        return bb;
    }
}
