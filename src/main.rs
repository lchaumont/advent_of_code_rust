use std::env;

mod year_2017;
mod year_2020;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run <year> <day>");
        return;
    }

    let year: u16 = args.get(1).and_then(|s| s.parse().ok()).unwrap();
    let day: u8 = args.get(2).and_then(|s| s.parse().ok()).unwrap();
    let test: bool = args.len() > 3 && args[3] == "test";

    let file_path = if test {
        format!("inputs/year_{}/day_{}_test.txt", year, day)
    } else {
        format!("inputs/year_{}/day_{}.txt", year, day)
    };

    println!("Reading file: {}", file_path);

    let input = std::fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Input not found: {file_path}"));

    println!("-----------");
    let time = std::time::Instant::now();

    match year {
        2017 => year_2017::run(day, &input),
        2020 => year_2020::run(day, &input),
        _ => eprintln!("Year {year} not implemented yet"),
    }

    let elapsed = time.elapsed();
    println!("-----------");
    println!("Elapsed time: {} ms", elapsed.as_millis());
}
