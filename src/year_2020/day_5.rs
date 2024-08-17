pub fn run(input: String) {
    let numbers: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    println!("Part 1: {:?}", part_1(&numbers));
    println!("Part 2: {:?}", part_2(&numbers));
}

fn part_1(boarding_passes: &Vec<Vec<char>>) -> u32 {
    let mut seat_ids: Vec<u32> = Vec::new();

    for boarding_passe in boarding_passes {
        let row_data: String = boarding_passe[..7].iter().map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => '0'
        }).collect();

        let column_data: String = boarding_passe[7..].iter().map(|x| match x {
            'L' => '0',
            'R' => '1',
            _ => '0'
        }).collect();

        let row_int = from_binary_string_to_int(&row_data);
        let column_int = from_binary_string_to_int(&column_data);

        let seat_id = row_int * 8 + column_int;
        seat_ids.push(seat_id);
    }

    match seat_ids.iter().max() {
        Some(val) => *val,
        None => 0
    }
}

fn part_2(boarding_passes: &Vec<Vec<char>>) -> u32 {
    let mut seat_ids: Vec<u32> = Vec::new();

    for boarding_passe in boarding_passes {
        let row_data: String = boarding_passe[..7].iter().map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => '0'
        }).collect();

        let column_data: String = boarding_passe[7..].iter().map(|x| match x {
            'L' => '0',
            'R' => '1',
            _ => '0'
        }).collect();

        let row_int = from_binary_string_to_int(&row_data);
        let column_int = from_binary_string_to_int(&column_data);

        let seat_id = row_int * 8 + column_int;
        seat_ids.push(seat_id);
    }

    seat_ids.sort();

    for index in 0..seat_ids.len() - 1 {
        let current = seat_ids[index];
        let next = seat_ids[index + 1];

        if current + 2 - next == 0 {
            return current + 1;
        }
    }

    0
}

fn from_binary_string_to_int(binary_string: &str) -> u32 {
    match u32::from_str_radix(binary_string, 2) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error converting binary string to int from {}", binary_string);
            0
        }
    }
}