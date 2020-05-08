use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let a: Vec<i32> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();

        input.clear();
        io::stdin().read_line(&mut input)?;
        let b: Vec<i32> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();

        if (
            (a[0] == b[0]) && (a[0] == a[1] + b[1]) ||
            (a[0] == b[1]) && (a[0] == a[1] + b[0]) ||
            (a[1] == b[0]) && (a[1] == a[0] + b[1]) ||
            (a[1] == b[1]) && (a[1] == a[0] + b[0])
        ) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    return Ok(());
}
