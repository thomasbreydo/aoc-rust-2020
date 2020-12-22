use std::vec;

fn setup(input: &str) -> Vec<usize> {
    let mut numbers = vec![0];
    for n in input.split('\n') {
        let n_as_int: usize = n.parse().unwrap();
        numbers.push(n_as_int);
    }
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);
    numbers
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
    let mut differences = Vec::new();
    for window in numbers.windows(2) {
        differences.push(window[1] - window[0]);
    }
    /* each value in n_arrangements_0_to_k is [a, b, c] where:
                    a = # arrangements ending with 1 difference
                    b = # arrangements ending with 2 difference
                    c = # arrangements ending with 3 difference
        for the subproblem 0..k
    */
    let mut n_arrangements_0_to_k = vec![vec![0, 0, 0]; differences.len()];
    n_arrangements_0_to_k[0][differences[0] - 1] = 1;
    for (i_minus_1, diff) in differences[1..].iter().enumerate() {
        let i = i_minus_1 + 1; // i_minus_1 is relative to differences[1..]

        // We can leave this jump as-is -> all of the previous arrangements are valid:
        n_arrangements_0_to_k[i][*diff - 1] = n_arrangements_0_to_k[i_minus_1].iter().sum();

        // We can also jump from previous arrangements that ended in 1-jump or 2-jump
        // as long as the new jump isn't too big
        for new_final_jump in (*diff + 1)..=(*diff + 2) {
            if new_final_jump > 3 {
                break;
            }
            n_arrangements_0_to_k[i][new_final_jump - 1] =
                n_arrangements_0_to_k[i_minus_1][new_final_jump - diff - 1];
        }
    }
    return n_arrangements_0_to_k.last().unwrap().iter().sum();
}
