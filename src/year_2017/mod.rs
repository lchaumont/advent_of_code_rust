pub mod day_1;

pub fn run(day: u8, input: &str) {
    match day {
        1  => dispatch(day, input, day_1::run),
        _  => eprintln!("Day {day} not implemented yet"),
    }
}

fn dispatch(
    day: u8,
    input: &str,
    solve: impl Fn(&str)
) {
    println!("── Day {day:02} ─────────────────────");
    solve(input);
}