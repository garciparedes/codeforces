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
        .map(|(_, x)| x.split_whitespace().map(|v| v == "1").collect::<Vec<_>>())
        .map(solve)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(case: Vec<bool>) -> String {
    let (mut smalls, mut bigs) = (0, 0);
    for small in case {
        if small {
            smalls += 1; 
        } else {
            bigs += 1;
        }
    }

    if condition(smalls, bigs) {
        return String::from("YES");
    } else {
        return String::from("NO");
    }
}

fn condition(smalls: usize, bigs: usize) -> bool {
    if (smalls + bigs * 2) % 2 != 0 {
        return false;
    }

    if bigs % 2 == 1 && smalls < 2 {
        return false;
    }

    return true;
}

