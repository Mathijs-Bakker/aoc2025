fn main() {
    let ranges = include_str!("../data/input.txt")
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
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
    let len = s.len();
    for size in 1..=len / 2 {
        if len.is_multiple_of(size) {
            let (unit, mut ok) = (&s[..size], true);

            for chunk in s.as_bytes().chunks(size) {
                if chunk != unit.as_bytes() {
                    ok = false;
                    break;
                }
            }

            if ok {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_number_11() {
        assert!(is_invalid_id(11))
    }

    #[test]
    fn test_invalid_numbers_111() {
        assert!(is_invalid_id(111))
    }

    #[test]
    fn test_invalid_numbers_() {
        assert!(is_invalid_id(824824824))
    }
}
