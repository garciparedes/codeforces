use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    for i in 0..5 {
        input.clear();    
        io::stdin().read_line(&mut input)?;

        match input.trim().split(" ").position(|x| x.parse::<u8>().unwrap() == 1) {
            Some(j) => {
                let result = (i as i8 - 2).abs() + (j as i8 - 2).abs();
                println!("{}", result);
            },
            None => continue,
        }
    }

    return Ok(());
}
