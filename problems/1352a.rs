use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
    
        let k = input.trim().len();

        let mut numbers: Vec<u32> = Vec::new();
        for (i, d) in input.trim().chars().enumerate() {
            let digit = d.to_digit(10).unwrap();
            if digit == 0 {
                continue;
            }
            numbers.push(digit * 10_u32.pow((k - i - 1) as u32));
        }
        let count = numbers.len();

        let result = numbers.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", count);
        println!("{}", result);

    }


    return Ok(());
}
