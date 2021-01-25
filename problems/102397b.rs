use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n: u8 = input.trim().parse().unwrap();

    let result = solve(n);

    println!("{} {}", result.0, result.1);

    return Ok(());
}

fn solve(n: u8) -> (u8, u8) {
    return (n, 1);
}
