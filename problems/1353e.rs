use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        io::stdin().read_line(&mut input)?;
    
        let result = solve(&input);

        println!("{}", result);

    }

    return Ok(());
}

fn solve(input: &String) -> String {
    let pre_parsed: Vec<&str> = input.trim().split("\n").take(2).collect();
    let pre_parsed_1: Vec<usize> = pre_parsed[0].trim().split(" ").map(|x| x.parse().unwrap()).collect(); 
    let k = pre_parsed_1[1];
    
    let s: Vec<i32> = pre_parsed[1].trim().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    let total = s.iter().filter(|&x| *x == 1).count() as i32;
    
    let mut result = total;
    for i in 0..k {
        let mut tmp: i32 = 0;
        for j in (i..s.len()).step_by(k) {
            tmp = cmp::max(0, tmp + s[j] * 2 - 1);
            result = cmp::min(result, total - tmp);
        }
    }

    return result.to_string();
}
