use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let cases = input.trim().parse::<u8>().unwrap();
 
    for _ in 0..cases {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.len() <= 10 {
            println!("{}", trimmed);
        } else {
            println!("{}{}{}", trimmed.chars().next().unwrap(), trimmed.len() - 2, trimmed.chars().last().unwrap())
        }
    }



    return Ok(());
}
