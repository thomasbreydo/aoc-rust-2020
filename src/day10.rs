fn setup(input: &str) -> Vec<usize> {
    let mut numbers = vec![0];
    for n in input.split('\n') {
        let n_as_int: usize = n.parse().unwrap();
        numbers.push(n_as_int);
    }
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);
    return numbers;
}
pub fn part1(input: String) {
    let numbers = setup(&input);
    let mut three_jumps = 0;
    let mut one_jumps = 0;
    for pair in numbers.windows(2) {
        match pair[1] - pair[0] {
            1 => one_jumps += 1,
            3 => three_jumps += 1,
            _ => (),
        }
    }
    println!("The answer is {}", one_jumps * three_jumps);
}

pub fn part2(input: String) {
    let numbers = setup(&input);
    println!("The answer is {}", n_arragements(&numbers));
}

fn n_arragements(numbers: &Vec<usize>) -> usize {
    for i in 1..numbers.len() {
        if okay_to_remove(&numbers, i) {
            let mut new_numbers = numbers.clone();
            new_numbers.swap_remove(i);
            return n_arragements(&new_numbers);
        }
    }
    return 1;
}

fn okay_to_remove(numbers: &Vec<usize>, index: usize) -> bool {
    if index == 0 || index == numbers.len() {
        return false;
    };
    return numbers[index + 1] - numbers[index - 1] <= 3;
}
// (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//    1   3  1  1  1  3   1   1   3   1   3   3
