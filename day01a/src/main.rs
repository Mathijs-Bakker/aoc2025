fn main() {
    let data = include_str!("../data/input.txt")
        .lines()
        .collect::<Vec<_>>();

    let (dial, zero_count) = get_password(data);
    println!("Dial: {dial}, Zero count: {zero_count}")
}

fn get_password(data: Vec<&str>) -> (i32, i32) {
    let mut dial: i32 = 50;
    let mut zero_count = 0;

    for n in data {
        let direction = n.chars().next().unwrap();
        let mut n = n
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        if n > 99 {
            n %= 100;
        }

        dial = if direction == 'L' {
            let d = dial - n;
            if d < 0 {
                100 + d
            } else {
                d
            }
        } else {
            let d = dial + n;
            if d > 99 {
                0 - (100 - d)
            } else {
                d
            }
        };

        if dial == 0 {
            zero_count += 1;
        }
    }

    (dial, zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_r10() {
        let v: Vec<&str> = vec!["R10"];
        assert_eq!(get_password(v), (60, 0));
    }

    #[test]
    fn test_add_r60() {
        let v: Vec<&str> = vec!["R60"];
        assert_eq!(get_password(v), (10, 0));
    }

    #[test]
    fn test_add_r160() {
        let v: Vec<&str> = vec!["R160"];
        assert_eq!(get_password(v), (10, 0));
    }

    #[test]
    fn test_subtract_l10() {
        let v: Vec<&str> = vec!["L10"];
        assert_eq!(get_password(v), (40, 0));
    }

    #[test]
    fn test_subtract_l60() {
        let v: Vec<&str> = vec!["L60"];
        assert_eq!(get_password(v), (90, 0));
    }

    #[test]
    fn test_subtract_l150() {
        let v: Vec<&str> = vec!["L150"];
        assert_eq!(get_password(v), (0, 1));
    }

    #[test]
    fn test_subtract_l460() {
        let v: Vec<&str> = vec!["L460"];
        assert_eq!(get_password(v), (90, 0));
    }
}
