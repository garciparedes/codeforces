use std::io;

fn main() -> io::Result<()> {
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let k: usize = input.trim().split(" ").last().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input)?;
    let scores: Vec<u8> = input.trim().split(" ").map(|s| s.parse().unwrap()).collect();
    let threshold = scores[k - 1];

    let mut count = 0;
    for score in scores {
        if (score == 0) || (score < threshold) {
            break;
        }
        count += 1;
    }

    println!("{}", count);
    
    return Ok(());
}
