use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let array: Vec<_> = input.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect(); 
    
        let result = solve(&array);

        println!("{}", result);

    }

    return Ok(());
}

fn solve(array: &[usize]) -> String {
    let (mut odd, mut even) = (0, 0);
    for (i, item) in array.iter().enumerate() {
        if i % 2 == 0 && item % 2 == 1 {
            odd += 1;
        } else if i % 2 == 1 && item % 2 == 0 {
            even += 1;
        }
    }
    if odd != even {
        return String::from("-1");
    }
    return odd.to_string();
}
