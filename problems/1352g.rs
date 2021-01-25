use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let t = input.trim().parse().unwrap();
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let n: u16 = input.trim().parse().unwrap();
        if n < 4 {
            println!("-1");
            continue;
        }
       
        let mut numbers: Vec<i16> = Vec::new();
        for i in (0..n).step_by(2).rev() {
            numbers.push((i as i16) + 1);
        }
        numbers.push(4);
        numbers.push(2);
        for i in (6..(n + 1)).step_by(2) {
            numbers.push(i as i16);
        } 

        let result = numbers.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", result);

    }


    return Ok(());
}
