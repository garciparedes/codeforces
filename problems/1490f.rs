use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

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
    let n = a.len();
    let mut counter = HashMap::new();
    for v in a {
        *counter.entry(v).or_insert(0) += 1;
    }

    let mut m = 0;
    let mut counts = vec![0; n + 1];
    for c in counter.values() {
        counts[*c] += 1;
        m += 1;
    }
    
    let mut ans = n;
    let mut cum = 0;
    for i in 1..=n {
        cum += counts[i];
        let tmp = n - i * counts[i] - (m - cum) * i; 
        if tmp < ans {
            ans = tmp;
        }
    }



    return ans;
}
