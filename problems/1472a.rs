use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .skip(1)
        .map(solve)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", ans);


    return Ok(());
}

fn solve(case: &str) -> String {
    let mut input = case.split_whitespace().map(|v| v.parse::<i32>().unwrap());
    let (mut w, mut h, n) = (input.next().unwrap(), input.next().unwrap(), input.next().unwrap());

    let mut cw = 1;
    while w % 2 == 0 && w / 2  > 0 {
        w /= 2;
        cw *= 2;
    } 
    
    let mut ch = 1;
    while h % 2 == 0 && h / 2 > 0 {
        h /= 2;
        ch *= 2;
    } 

    if cw * ch >= n {
        return String::from("YES");
    } else {
        return String::from("NO");
    }

}
