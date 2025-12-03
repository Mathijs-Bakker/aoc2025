fn main() {
    let ranges = include_str!("../data/input.txt")
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            println!("{a:?} - {b:?}");
            (a.trim().parse().unwrap(), b.trim().parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    println!("Solution: {}", find_invalid_ids(ranges).iter().sum::<u64>())
}

fn find_invalid_ids(ranges: Vec<(u64, u64)>) -> Vec<u64> {
    let mut ids: Vec<u64> = Vec::new();

    for rng in ranges {
        for id in rng.0..=rng.1 {
            if is_invalid_id(id) {
                ids.push(id);
            }
        }
    }

    ids
}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let mid = s.len() / 2;
    let (a, b) = s.split_at(mid);
    if a == b {
        return true;
    }
    false
}
