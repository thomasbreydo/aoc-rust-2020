// Days
pub mod day10;
pub mod day15;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    match day {
        10 => (day10::part1, day10::part2),
        15 => (day15::part1, day15::part2),
        _ => {
            println!("Unknown day: {}", day);
            (noop, noop)
        }
    }
}
