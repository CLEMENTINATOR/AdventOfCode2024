type Num = u32;

enum Order {
    Asc,
    Desc,
}
fn are_my_tokens_safe(tokens: Vec<u32>) -> bool {
    let token_window = tokens.windows(2);
    let mut ascending: Option<Order> = None;

    for window in token_window {
        match window {
            &[first, second] => {
                if first.abs_diff(second) == 0 || first.abs_diff(second) > 3 {
                    return false;
                }
                match ascending {
                    None => {
                        ascending = if first > second {
                            Some(Order::Desc)
                        } else {
                            Some(Order::Asc)
                        }
                    }
                    Some(Order::Asc) => {
                        if first > second {
                            return false;
                        }
                    }
                    Some(Order::Desc) => {
                        if second > first {
                            return false;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    true
}

fn is_my_line_safe(line: &str) -> bool {
    let tokens: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    are_my_tokens_safe(tokens)
}

pub fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|line| if is_my_line_safe(line) { 1 } else { 0 })
        .sum()
}

pub fn part2(input: &str) -> Num {
    input
        .lines()
        .map(|line| {
            if is_my_line_safe(line) {
                1
            } else {
                let tokens: Vec<u32> = line
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
                for i in 0..tokens.len() {
                    let (left, right) = tokens.split_at(i);
                    let temp_tokens = [left, &right[1..]].concat();
                    if are_my_tokens_safe(temp_tokens) {
                        return 1;
                    }
                }
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = indoc::indoc! {"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"};

    #[test]
    fn example() {
        assert_eq!(part1(EXAMPLE), 2);
        assert_eq!(part2(EXAMPLE), 4);
    }

    #[test]
    fn run_part1() {
        let input = crate::utils::get_day_input!();
        let output = part1(&input);
        println!("Part 1: {}", output);
        assert_eq!(output, 359);
    }

    #[test]
    fn run_part2() {
        let input = crate::utils::get_day_input!();
        let output = part2(&input);
        println!("Part 2: {}", output);
        assert_eq!(output, 418);
    }
}
