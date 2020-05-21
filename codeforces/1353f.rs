use std::io;
use std::cmp;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        let shape: Vec<usize> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let (n, m) = (shape[0], shape[1]);
        let mut matrix: Vec<Vec<i64>> = Vec::new();
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input)?;
            let row: Vec<i64> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
            matrix.push(row);
        }
        let result = solve(&mut matrix, n, m);

        println!("{}", result);

    }

    return Ok(());
}

fn solve(matrix: &mut Vec<Vec<i64>>, n: usize, m: usize) -> String {
   
    let mut result = 10_i64.pow(18);
    let base = matrix[0][0];
    for x in 0..n {
        for y in 0..m {
            
            let b = matrix[x][y] - (x + y) as i64;
            if base < b {
                continue;
            }
            matrix[0][0] = b;
            
            let mut working = vec![vec![10_i64.pow(18); m]; n];
            working[0][0] = base - b;
            for i in 0..n {
                for j in 0..m {
                    let b = matrix[0][0] + (i + j) as i64;
                    if matrix[i][j] < b {
                        continue;
                    }
                    
                    if i > 0 {
                        working[i][j] = cmp::min(working[i][j], working[i - 1][j] + matrix[i][j] - b);
                    }
                    
                    if j > 0 {
                        working[i][j] = cmp::min(working[i][j], working[i][j - 1] + matrix[i][j] - b);
                    }
                }
            } 
            result = cmp::min(result, working[n - 1][m - 1]);
        }
    }


    return result.to_string();
}
