use std::env;
use std::io::Read;

mod year_2020;
// Add more year modules as needed

fn read_input_file(year: &str, day: &str, test: bool) -> Result<String, std::io::Error> {
    let file_path = if test {
        format!("inputs/year_{}/day_{}_test.txt", year, day)
    } else {
        format!("inputs/year_{}/day_{}.txt", year, day)
    };

    println!("Reading file: {}", file_path);

    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run <year> <day>");
        return;
    }

    let year = &args[1];
    let day = &args[2];
    let test: bool = args.len() > 3 && args[3] == "test";

    let read_file = read_input_file(year, day, test);

    println!("-----------");
    let time = std::time::Instant::now();

    match read_file {
        Ok(contents) => {
            match year.as_str() {
                "2020" => match day.as_str() {
                    "1" => year_2020::day_1::run(contents),
                    "2" => year_2020::day_2::run(contents),
                    "3" => year_2020::day_3::run(contents),
                    "4" => year_2020::day_4::run(contents),
                    "5" => year_2020::day_5::run(contents),
                    "6" => year_2020::day_6::run(contents),
                    "7" => year_2020::day_7::run(contents),
                    "8" => year_2020::day_8::run(contents),
                    "9" => year_2020::day_9::run(contents),
                    "10" => year_2020::day_10::run(contents),
                    "11" => year_2020::day_11::run(contents),
                    "12" => year_2020::day_12::run(contents),
                    "13" => year_2020::day_13::run(contents),
                    "14" => year_2020::day_14::run(contents),

                    _ => eprintln!("Invalid day: {}", day),
                },

                _ => eprintln!("Invalid year: {}", year),
            }
        },
        Err(e) => eprintln!("Error reading file: {}", e)
    }

    let elapsed = time.elapsed();
    println!("-----------");
    println!("Elapsed time: {} ms", elapsed.as_millis());
}