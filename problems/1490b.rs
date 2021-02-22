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
        .map(|line| line.trim().split_whitespace().map(|value| value.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .map(solve)
        .map(|solution| solution.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(a: Vec<usize>) -> usize {
    let mut c = vec![0; 3];
    for v in a {
        c[v % 3] += 1;
    }

    let target = c.iter().sum::<usize>() / 3;
    
    let mut ans = 0;
    while !c.iter().all(|v| *v == target) {
        for i in 0..3 {
            if c[i] > target {
                let tmp = c[i] - target;
                ans += tmp;
                c[i] -= tmp;
                c[(i + 1) % 3] += tmp;
            }
        } 
    }

    return ans;
}
