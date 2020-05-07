use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<u8>().unwrap();

    let mut count = 0;
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        if input.contains("++") {
            count += 1;
        } else {
            count -= 1;
        }
    }
    println!("{}", count);

    return Ok(());
}
