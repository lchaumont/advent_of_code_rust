pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_9;
pub mod day_8;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;

pub fn run(day: u8, input: &str) {
    match day {
        1  => dispatch(day, input, day_1::run),
        2  => dispatch(day, input, day_2::run),
        3  => dispatch(day, input, day_3::run),
        4  => dispatch(day, input, day_4::run),
        5  => dispatch(day, input, day_5::run),
        6  => dispatch(day, input, day_6::run),
        7  => dispatch(day, input, day_7::run),
        9  => dispatch(day, input, day_9::run),
        8  => dispatch(day, input, day_8::run),
        10  => dispatch(day, input, day_10::run),
        11  => dispatch(day, input, day_11::run),
        12  => dispatch(day, input, day_12::run),
        13  => dispatch(day, input, day_13::run),
        14  => dispatch(day, input, day_14::run),
        15  => dispatch(day, input, day_15::run),
        _  => eprintln!("Day {day} not implemented yet"),
    }
}

fn dispatch(
    day: u8,
    input: &str,
    solve: impl Fn(&str)
) {
    println!("── Day {day:02} ─────────────────────");
    solve(input.into());
}