use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let amount: u8 = input.trim().parse().unwrap();
    let result = solve(amount);

    println!("{}", result);

    return Ok(());
}

fn solve(amount: u8) -> u8 {
    return amount * 2;
}
