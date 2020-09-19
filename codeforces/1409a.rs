
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;

        let a_b: Vec<_> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let a = a_b[0];
        let b = a_b[1];
        let ans = (((a - b).abs() as f64) / 10.0).ceil();
        println!("{}", ans);



    }

    return Ok(());
}
