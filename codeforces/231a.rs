use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: usize = input.trim().parse().unwrap();

    let mut count: usize = 0;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let tmp = input
            .trim()
            .split(" ")
            .map(|x| x.parse::<u8>().unwrap())
            .sum::<u8>();

        if tmp < 2 {
            continue;
        }
        count += 1
    }
    println!("{}", count);

    return Ok(());
}
