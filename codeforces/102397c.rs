use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    
    let x_y: Vec<i16> = input.trim().split(" ").map(|x| x.parse::<i16>().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input)?;

    let result = solve(x_y[0], x_y[1], input);

    println!("{} {}", result.0, result.1);

    return Ok(());
}

fn solve(mut x: i16, mut y: i16, path: String) -> (i16, i16){
    for action in path.chars() {
        match action {
            'U' => y += 1,
            'D' => y -= 1,
            'R' => x += 1,
            'L' => x -= 1,
            _ => (),
        }
    }
    return (x, y);
}
