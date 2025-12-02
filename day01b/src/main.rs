fn main() {
    let data = include_str!("../data/input.txt")
        .lines()
        .collect::<Vec<_>>();

    let (dial, zero_count) = get_password(data, 50);
    println!("Dial: {dial}, Zero count: {zero_count}")
}

fn get_password(data: Vec<&str>, mut dial: i32) -> (i32, i32) {
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
            zero_count += n / 100;
            n %= 100;
        }

        dial = if direction == 'L' {
            let d = dial - n;
            if d < 0 {
                if dial != 0 {
                    zero_count += 1;
                }
                100 + d
            } else {
                d
            }
        } else {
            let d = dial + n;
            if d > 99 {
                if d != 100 && dial != 0 {
                    zero_count += 1;
                }
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
    fn test_add_5_to_0() {
        let v: Vec<&str> = vec!["R5"];
        assert_eq!(get_password(v, 0), (5, 0));
    }

    #[test]
    fn test_subtract_5_from_0() {
        let v: Vec<&str> = vec!["L5"];
        assert_eq!(get_password(v, 0), (95, 0));
    }

    #[test]
    fn test_subtract_50_from_50() {
        let v: Vec<&str> = vec!["L50"];
        assert_eq!(get_password(v, 50), (0, 1));
    }

    #[test]
    fn test_add_48_to_52() {
        let v: Vec<&str> = vec!["R48"];
        assert_eq!(get_password(v, 52), (0, 1));
    }

    #[test]
    fn test_add_r10() {
        let v: Vec<&str> = vec!["R10"];
        assert_eq!(get_password(v, 50), (60, 0));
    }

    #[test]
    fn test_add_r60() {
        let v: Vec<&str> = vec!["R60"];
        assert_eq!(get_password(v, 50), (10, 1));
    }

    #[test]
    fn test_add_r160() {
        let v: Vec<&str> = vec!["R160"];
        assert_eq!(get_password(v, 50), (10, 2));
    }

    #[test]
    fn test_subtract_l10() {
        let v: Vec<&str> = vec!["L10"];
        assert_eq!(get_password(v, 50), (40, 0));
    }

    #[test]
    fn test_subtract_l60() {
        let v: Vec<&str> = vec!["L60"];
        assert_eq!(get_password(v, 50), (90, 1));
    }

    #[test]
    fn test_subtract_l150() {
        let v: Vec<&str> = vec!["L150"];
        assert_eq!(get_password(v, 50), (0, 2));
    }

    #[test]
    fn test_subtract_l460() {
        let v: Vec<&str> = vec!["L460"];
        assert_eq!(get_password(v, 50), (90, 5));
    }
}
