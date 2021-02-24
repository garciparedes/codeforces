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
        .map(|line| line.trim().split_whitespace().map(|value| value.parse::<u8>().unwrap()).collect::<Vec<_>>())
        .map(solve)
        .map(|solution| solution.into_iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}


fn solve(permutation: Vec<u8>) -> Vec<u8> {
    let mut ans = vec![0; permutation.len()];
    helper(&permutation, &mut ans, 0);
    return ans;
}


fn helper(data: &[u8], height: &mut [u8], depth: u8) {
    if data.is_empty() {
        return;
    }
    let mut imax = 0;
    for i in 0..data.len() {
        if data[imax] < data[i] {
            imax = i;
        }
    }

    height[imax] = depth;
    helper(&data[..imax], &mut height[..imax], depth + 1);
    helper(&data[imax + 1..], &mut height[imax + 1..], depth + 1);
}
