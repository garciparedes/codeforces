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
    let parsed: Vec<u32> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    let (n, m) = (parsed[0], parsed[1]);
    
    let mut value = 0;
    if n == 2 {
        value = m;    
    } else if n > 2 {
        value = 2 * m;
    }

    return value.to_string();
}
