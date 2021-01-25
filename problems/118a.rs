use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    input = input.trim().to_lowercase().to_string();
    
    let decomposed = input.chars().filter(|x| ['a', 'e', 'i', 'o', 'u', 'y'].iter().all(|y| y != x));
    let result = decomposed.fold(String::new(), |a, b| format!("{}.{}", a, b));

    println!("{}", result);

    return Ok(());
}
