use std::io::prelude::*;
use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let cubes = generate_cubes(1_000_000_000_000);

    let ans = buffer
        .trim()
        .split('\n')
        .skip(1)
        .map(|line| line.trim().parse::<u64>().unwrap())
        .map(|x| solve(x, &cubes))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn generate_cubes(maximum: u64) -> HashSet<u64> {
    let mut current = (1, 1);

    let mut ans = HashSet::new();
    while current.1 < maximum {
       ans.insert(current.1);
       current = (current.0 + 1, (current.0 + 1) * (current.0 + 1) * (current.0 + 1))
    } 
    return ans;
}

fn solve(x: u64, cubes: &HashSet<u64>) -> String {
    for cube in cubes {
        if *cube < x && cubes.contains(&(x - *cube)) {
            return String::from("YES");
        }
    }
    return String::from("NO");
}
