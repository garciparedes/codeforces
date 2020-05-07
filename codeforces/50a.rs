use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let numbers: Vec<u16> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<u16>().unwrap())
        .take(2)
        .collect();
    
    let (m, n) = (numbers[0], numbers[1]);

    let result = m * n / 2;
    println!("{}", result);

    return Ok(());
}
