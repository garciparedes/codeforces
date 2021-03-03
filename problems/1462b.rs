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
        .map(|line| line.trim())
        .map(solve)
        .map(|solution| if solution { String::from("YES") } else { String::from("NO") })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);

    return Ok(());
}

fn solve(s: &str) -> bool {
    if s.starts_with("2020") {
        return true;
    }

    if s.starts_with("202") && s.ends_with("0") {
        return true;
    }

    if s.starts_with("20") && s.ends_with("20") {
        return true;
    }

    if s.starts_with("2") && s.ends_with("020") {
        return true;
    }

    if s.ends_with("2020") {
        return true;
    }

    return false;
}
