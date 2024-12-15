type Num = u32;

pub fn part1(input: &str) -> Num {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let a: Vec<u32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        list1.push(a[0]);
        list2.push(a[1]);
    }

    list1.sort();
    list2.sort();

    let mut res = 0;
    for i in 0..list1.len() {
        let diff: i32 = (list1[i] as i32) - (list2[i] as i32);
        res += diff.abs();
    }
    return res.try_into().unwrap();
}

pub fn part2(input: &str) -> Num {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
3   4
4   3
2   5
1   3
3   9
3   3
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 0);
        assert_eq!(part2(EXAMPLE), 0);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 0);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 0);
    }
}
