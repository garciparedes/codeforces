use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split("\n")
        .skip(1)
        .map(|line| line.trim().parse::<usize>().unwrap())
        .map(solve)
        .map(|solution| solution.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(x: usize) -> i32 {
    let mut available = vec![true; 10];
    
    
    if !helper(&mut available, 9, 0, x) {
        return -1;

    }
    let mut ans = 0;
    for i in 1..=9 {
        if !available[i] {
            ans *= 10;
            ans += i as i32;
        }
    } 
    return ans;
}


fn helper(available: &mut [bool], max: usize, current: usize, target: usize) -> bool {
    if current > target {
        return false;
    }
    if current == target {
        return true;
    }

    for i in (1..=max).rev() {
        if !available[i] {
            continue;
        }
        available[i] = false;
        if helper(available, i - 1, current + i, target) {
            return true;
        }
        available[i] = true;
    }
    return false;
}
