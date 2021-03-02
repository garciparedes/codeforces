use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .skip(2)
        .step_by(2)
        .map(|line| line.trim().split_whitespace().map(|value| value.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .map(solve)
        .map(|solution| solution.into_iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(a: Vec<u32>) -> Vec<u32> {
    let n = a.len();
    let mut ans = Vec::new();
    for i in 0..n / 2 {
        ans.push(a[i]);
        ans.push(a[n - 1 - i]);
    }
    if n % 2 == 1 {
        ans.push(a[n / 2])
    }
    return ans;
}
