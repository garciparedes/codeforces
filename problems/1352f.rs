use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let numbers: Vec<u8> = input.trim().split(" ").map(|x| x.parse().unwrap()).take(3).collect();
        let (n0, mut n1, n2) = (numbers[0], numbers[1], numbers[2]); 

        let mut result = String::new();
        if n1 > 0 && n1 % 2 == 0 {
            result.push('0');
            n1 -= 1;
        }
        if n2 > 0 || n1 > 0 {
            result.push('1');
        }
        for _ in 0..n2 {
            result.push('1');
        }
        for i in 0..n1 {
            if i % 2 == 0 {
                result.push('0');
            } else {
                result.push('1');
            }
        }
        if n0 > 0 && (result.len() == 0 || result.chars().last().unwrap() == '1') {
            result.push('0');
        }
        for _ in 0..n0 {
            result.push('0');
        }
        println!("{}", result);
    }


    return Ok(());
}

