use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
    
        let result = solve(&input);

        println!("{}", result);

    }

    return Ok(());
}

fn solve(input: &String) -> String {
    let n: u64 = input.trim().parse().unwrap();
    let mut result: u64 = 0;
    for i in 1..(n / 2 + 1) {
        result += i.pow(2) * 8;
    }
    return result.to_string();
}
