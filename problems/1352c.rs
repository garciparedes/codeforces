use std::io;



fn main() -> io::Result<()> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let n_k: Vec<u32> = input.trim().split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
        let (n, k) = (n_k[0], n_k[1]);
       
        let ops = (k - 1) / (n - 1);
        let count = (n - 1) * ops;
        let mut current = n * ops;

        current += k - count;
        println!("{}", current);

    }


    return Ok(());
}
