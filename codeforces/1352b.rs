use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let n_k: Vec<i32> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let (n, k) = (n_k[0], n_k[1]);

        if (n >= k) && ((n % k) % 2 == 0 || (n / k > 1 && (k - 1) % 2 == 0)) {
            let mut numbers = vec![n / k; k as usize];
            numbers[0] += n % k;
            if (numbers.len() > 1) && ((numbers[0] % 2) != (numbers[1] % 2)) {
                for i in 1..k {
                    numbers[i as usize] += ((i % 2 == 0) as i32) * 2 - 1;
                }
            }
            
            let result = numbers.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
            println!("YES");
            println!("{}", result);
        } else {
            println!("NO");
        }
    }


    return Ok(());
}
