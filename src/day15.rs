use std::collections::HashMap;

pub fn part1(input: String) {
    let numbers = parse_input(input);
    println!("The 2020th number is {}", value_2020(numbers));
}

pub fn part2(input: String) {
    let numbers = parse_input(input);
    println!("The 30000000th number is {}", value_n(numbers, 30000000));
}

fn parse_input(input: String) -> Vec<usize> {
    return input.split(',').map(|n| n.parse().unwrap()).collect();
}

fn value_2020(start: Vec<usize>) -> usize {
    value_n(start, 2020)
}

fn value_n(mut start: Vec<usize>, n: usize) -> usize {
    let mut last_seen = HashMap::new();
    for (i, n) in start.iter().enumerate() {
        last_seen.insert(*n, i);
    }
    for i in start.len()..n {
        let prev_index = i - 1;
        let prev_value = &start[prev_index];
        match last_seen.get(prev_value) {
            Some(last_seen_index) => {
                let new_value = prev_index - last_seen_index;
                start.push(new_value);
            }
            None => start.push(0),
        };
        last_seen.insert(start[prev_index], prev_index);
    }
    *start.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{value_2020, value_n};

    #[test]
    fn test_value_2020() {
        assert_eq!(value_2020(vec![0, 3, 6]), 436);
        assert_eq!(value_2020(vec![1, 3, 2]), 1);
        assert_eq!(value_2020(vec![2, 1, 3]), 10);
        assert_eq!(value_2020(vec![1, 2, 3]), 27);
        assert_eq!(value_2020(vec![2, 3, 1]), 78);
        assert_eq!(value_2020(vec![3, 2, 1]), 438);
        assert_eq!(value_2020(vec![3, 1, 2]), 1836);
    }

    #[test]
    fn test_value_n() {
        assert_eq!(value_n(vec![0, 3, 6], 30000000), 175594);
        assert_eq!(value_n(vec![1, 3, 2], 30000000), 2578);
        assert_eq!(value_n(vec![2, 1, 3], 30000000), 3544142);
        assert_eq!(value_n(vec![1, 2, 3], 30000000), 261214);
        assert_eq!(value_n(vec![2, 3, 1], 30000000), 6895259);
        assert_eq!(value_n(vec![3, 2, 1], 30000000), 18);
        assert_eq!(value_n(vec![3, 1, 2], 30000000), 362);
    }
}
