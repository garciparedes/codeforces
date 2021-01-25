use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
    
        let value: i32 = input.trim().split(" ").map(|x| x.parse::<i32>().unwrap()).sum();

        println!("{}", value);

    }


    return Ok(());
}
