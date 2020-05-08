use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let count = input.trim().parse().unwrap();
    for _ in 0..count {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let mut visited = HashSet::new();
        let mut cost = 0;
        let (mut x, mut y) = (0, 0);
        let mut current = (x, y);
        input.trim().to_uppercase().chars().for_each(|c| {
            match c {
                'N' => {
                    current = (x, y + 1);
                    y += 2;
                },
                'S' => {
                    current = (x, y - 1);
                    y -= 2;
                },
                'E' => {
                    current = (x + 1, y);
                    x += 2;
                },
                'W' => {
                    current = (x - 1, y);
                    x -= 2;
                },
                _ => return
            }
            if !visited.insert(current) {
                cost += 1;
            } else {
                cost += 5;
            }
        });
        println!("{}", cost);
    }


    return Ok(());
}
