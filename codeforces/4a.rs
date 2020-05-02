use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let weight = input.trim().parse::<u8>().unwrap();
    
    if weight > 2 && weight % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }

    return Ok(());
}
