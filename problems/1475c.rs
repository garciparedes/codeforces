use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut iterable = buffer
        .trim()
        .split('\n')
        .skip(1);

    while let Some(metadata) = iterable.next() {
        let metadata: Vec<_> = metadata
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let a: Vec<_> = iterable
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let b: Vec<_> = iterable
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let pairs: Vec<_> = a
            .into_iter()
            .zip(b.into_iter())
            .collect();

        let ans = solve(pairs,  metadata[0], metadata[1]);
        println!("{}", ans);
    }

    return Ok(());
}

fn solve(mut pairs: Vec<(usize, usize)>, n: usize, m: usize) -> u128 {
    let k = pairs.len();
    pairs.sort_unstable();
    let mut indexer = vec![0; n];
    let mut counter = vec![0; m];
    for (i, pair) in pairs.iter().enumerate() {
        if indexer[pair.0 - 1] == 0 {
            indexer[pair.0 - 1] = i;
        }
        counter[pair.1 - 1] += 1;
    }

    let mut ans: u128 = 0;
    for pair in pairs {
        let mut tmp = pair.0;
        while tmp < indexer.len() && indexer[tmp] == 0 {
            tmp += 1;
        }
        if tmp >= n {
            break;
        }
        counter[pair.1 - 1] -= 1;
        ans += (k - indexer[tmp] - counter[pair.1 - 1]) as u128;
    } 
    return ans;
}
