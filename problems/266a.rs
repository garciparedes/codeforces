use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    input.clear();
    io::stdin().read_line(&mut input)?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    
    let result = solve(&input);
    println!("{}", result);

    return Ok(());
}

fn solve(input: &String) -> String {
    let mut counter: u8 = 0;

    for w in input.chars().collect::<Vec<char>>().windows(2) {
        let (a, b) = (w[0], w[1]);
        if a == b {
            counter += 1;
        }
    }
    return counter.to_string();
}
