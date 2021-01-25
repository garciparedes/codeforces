use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let parsed: Vec<i32> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let k = parsed[1];

        input.clear();
        io::stdin().read_line(&mut input)?;
        let numbers: Vec<i32> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();

        let result = solve(&numbers, k);

        println!("{}", result);

    }

    return Ok(());
}

fn solve(numbers: &[i32], k: i32) -> String {
    let odd_count: i32 = numbers.iter().map(|x| (x % 2 == 1) as i32).sum();
    let even_count = numbers.len() as i32 - odd_count;
   
    if odd_count == 0 {
        return "No".to_string();
    }

    let mut current = k;
    
    if current >= odd_count {
        current -= if odd_count % 2 == 1 { odd_count } else { odd_count - 1 };
    } else {
        current = (current % 2 == 0) as i32;
    }

    if even_count >= current {
        return "Yes".to_string();
    } else {
        return "No".to_string();
    }
}
