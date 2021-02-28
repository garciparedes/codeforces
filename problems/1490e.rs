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
        .map(|line| line.trim().split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>())
        .map(solve)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(a: Vec<usize>) -> String {
    let n = a.len();

    let mut cum = a.clone();
    cum.sort_unstable();
    let sorted = cum.clone();

    let mut j = 0;
    for i in 1..n {
        if cum[i - 1] < cum[i] {
            j = i;
        } 
        cum[i] += cum[i - 1];
        if cum[i] > sorted[sorted.len() - 1] {
            break;
        }
    }
    let threshold = sorted[j];
    
    let mut indexes = Vec::new();
    for i in 0..n {
        if threshold <= a[i] {
            indexes.push((i + 1).to_string());
        }  
    }

    return format!("{}\n{}", indexes.len(), indexes.join(" "));
}

