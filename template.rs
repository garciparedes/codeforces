use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
    
        let result = solve(&input.trim());

        println!("{}", result);

    }

    return Ok(());
}

fn solve(input: &str) -> String {
    let mut result = String::new();

    result.push_str(input);

    return result;
}
