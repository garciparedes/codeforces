use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let mut counter: i8 = 0;
    let mut current: char = '1';
    for player in input.trim().chars(){
        if player != current{
            current = player;
            counter = 1;
            continue;
        } 
        counter += 1;
        if counter >= 7 {
            println!("YES");
            return Ok(());
        }
    };


    println!("NO");

    return Ok(());
}
