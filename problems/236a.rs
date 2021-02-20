use std::io::prelude::*;
use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let uniques: HashSet<_> = buffer.trim().chars().collect();

    if uniques.len() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
    return Ok(());
}
