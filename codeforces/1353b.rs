use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        io::stdin().read_line(&mut input)?;
        io::stdin().read_line(&mut input)?;
        
        let result = solve(&input);

        println!("{}", result);

    }

    return Ok(());
}

fn solve(input: &String) -> String {
    let parsed: Vec<Vec<u16>> = input
        .trim()
        .split("\n")
        .map(|row| row.trim().split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();
    let (n, k, mut a, mut b) = (
        parsed[0][0] as usize, 
        parsed[0][1] as usize, 
        parsed[1].clone(), 
        parsed[2].clone()
    );

    a.sort();
    b.sort();

    let mut counter: u16 = 0;
    for i in 0..k {
        if a[i] >= *b.iter().rev().nth(i).unwrap() {
            counter += a[i];
        } else {
            counter += b.iter().rev().nth(i).unwrap();
        }
    }
    counter += a.iter().rev().take(n - k).sum::<u16>();

    return counter.to_string();
}
