use std::io;
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a)?;
    io::stdin().read_line(&mut b)?;

    match a.to_lowercase().cmp(&b.to_lowercase()) {
        Ordering::Greater => println!("1"),
        Ordering::Equal => println!("0"),
        Ordering::Less => println!("-1")
    }

    return Ok(());
}
