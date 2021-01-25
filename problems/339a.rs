use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let mut numbers: Vec<&str> = input.trim().split("+").collect();
    numbers.sort();

    println!("{}", numbers.join("+"));

    return Ok(());
}
