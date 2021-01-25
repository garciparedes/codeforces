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
        let numbers: Vec<u32> = input.trim().split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
        
        let mut special = 0;
        for (i, target) in numbers.clone().into_iter().enumerate() {
            let mut cum = 0;
            let mut vals = 0;
            for (j, current) in numbers.clone().into_iter().enumerate() {
                if i == j {
                    cum = 0;
                    vals = 0;
                    continue;
                }
                cum += current;
                vals += 1;
                while cum > target {
                    cum -= numbers[j + 1 - vals];
                    vals -= 1;
                }
                if cum == target && vals > 1 {
                    special += 1;
                    break;
                } 
            }
        }



        println!("{}", special);

    }


    return Ok(());
}
