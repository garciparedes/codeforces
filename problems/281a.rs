use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let result = capitalize(input.trim());
    println!("{}", result);
    
    return Ok(());
}

fn capitalize(word: &str) -> String {
    let mut characters: Vec<char> = word.chars().collect();
    characters[0] = characters[0].to_uppercase().nth(0).unwrap();
    return characters.iter().collect();
}
