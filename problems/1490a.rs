use std::io::prelude::*;
use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;


    let ans = buffer
        .trim()
        .split('\n')
        .skip(2)
        .step_by(2)
        .map(|line| line.trim().split(' ').map(|num| num.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .map(solve)
        .map(|solution| solution.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(a: Vec<u32>) -> u32 {
    let mut ans = 0;

    for i in 0..a.len() - 1 {
        let mut tmp = cmp::max(a[i], a[i + 1]) as f32 / cmp::min(a[i], a[i + 1]) as f32;
        while tmp > 2.0 {
            tmp /= 2.0;
            ans += 1;
        }
    }

    return ans;
}
