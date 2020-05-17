use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
    
        let result = solve(&input);

        println!("{}", result);

    }

    return Ok(());
}

#[derive(Eq)]
struct Range {
    l: usize,
    r: usize,
}

impl Range {
    fn new(l: usize, r: usize) -> Range {
        return Range {l: l, r:r}
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.r - self.l).cmp(&(other.r - other.l)) {
            Ordering::Equal => return other.l.partial_cmp(&self.l).unwrap(),
            other => return other,
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        return (self.l == other.l) && (self.r == other.r);
    }
}


fn solve(input: &String) -> String {
    let n: usize = input.trim().parse().unwrap();

    let mut array: Vec<u32> = vec![0; n]; 
    let mut segments: BinaryHeap<Range> = BinaryHeap::new();
    segments.push(Range::new(0, n-1));
    for i in 0..n {
        let range = segments.pop().unwrap();
        let (l, r) = (range.l, range.r);
        let mut pos = ((l + r) / 2) as usize;
        if (r - l + 1) % 2 == 0 {
            pos = ((l + r - 1) / 2) as usize;
        }
        
        array[pos] = i as u32 + 1;
        if l < pos {
            segments.push(Range::new(l, pos - 1));
        }
        if pos < r {
            segments.push(Range::new(pos + 1, r));
        } 
    }

    let result = array
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    return result;
}
