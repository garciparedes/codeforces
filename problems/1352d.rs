use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        input.clear();
        io::stdin().read_line(&mut input)?;
        let mut candies: Vec<u32> = input.trim().split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
        
        
        let (mut a, mut b, mut count, mut prev) = (0, 0, 0, 0);
        let mut current = true;
        while !candies.is_empty() {
            count += 1;
            let mut now = 0;
            if current {
                while (now <= prev) && !candies.is_empty(){
                    now += candies[0];
                    candies.remove(0);
                }
                a += now;
                prev = now;
            } else {
                while (now <= prev) && !candies.is_empty(){
                    now += candies.pop().unwrap();
                }
                b += now;
                prev = now;
            }

            current = !current;
        }


        println!("{} {} {}", count, a, b);

    }


    return Ok(());
}
