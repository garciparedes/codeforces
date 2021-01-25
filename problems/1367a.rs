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
    let n = input.len();
    let pair = n % 2 == 0;
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 || (pair && i == (n - 1)) {
            result.push(c);
        }
    }
    return result;
}
