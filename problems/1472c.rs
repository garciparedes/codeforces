use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .skip(1)
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, x)| x.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .map(solve)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(a: Vec<usize>) -> usize {
    let mut memo = vec![0; a.len()];

    for i in (0..a.len()).rev() {
       memo[i] += a[i];
       if i + a[i] < memo.len() {
           memo[i] += memo[i + a[i]];
       }
    }

    return memo.into_iter().max().unwrap_or(0);
}

