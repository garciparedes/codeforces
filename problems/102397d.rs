use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let x_n: Vec<u32> = input.trim().split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
    
    input.clear();
    io::stdin().read_line(&mut input)?;
    let amount: Vec<u32> = input.trim().split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

    let result = solve(x_n[1], &amount);

    println!("{}", result);


    return Ok(());
}

fn solve(x: u32, amount: &[u32]) -> i32{
    let mut best  = - 1;
    let mut i = 0;
    let mut cumulated = 0;
    for (j, &v) in amount.iter().enumerate() {
        cumulated += v;
        while cumulated >= x {
            if best == -1 || best>  (j - i + 1) as i32 {
                best = (j - i + 1) as i32;
            }
            cumulated -= amount[i];
            i += 1;
        }
    } 
    return best;
}
