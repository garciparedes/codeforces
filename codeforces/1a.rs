use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let numbers: Vec<u64> = input.trim().split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
    let (m, n, a) = (numbers[0], numbers[1], numbers[2]);
    
    let x = m / a + ((m % a != 0) as u64);
    let y = n / a + ((n % a != 0) as u64);
    let flagstones = x * y;

    println!("{}", flagstones);

    return Ok(());
}
