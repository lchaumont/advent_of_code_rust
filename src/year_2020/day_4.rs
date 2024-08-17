use regex::Regex;
use std::collections::HashMap;

pub fn run(input: String) {
    let passports: Vec<Vec<&str>> = input
        .split("\n\r\n")
        .map(|x| x.split_whitespace().collect())
        .collect();

    println!("Part 1: {:#?}", part_1(&passports));
    println!("Part 2: {:#?}", part_2(&passports));
}

fn part_1(passports: &Vec<Vec<&str>>) -> u32 {
    let mut result = 0;

    let mandatory_fields: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    'PASSPORTS_LOOP: for passport in passports {
        let keys: Vec<&str> = passport.iter().map(|x| x.split(':').collect::<Vec<&str>>()[0]).collect();

        for field in mandatory_fields {
            if !keys.contains(&field) {
                continue 'PASSPORTS_LOOP;
            }
        }

        result += 1;
    }

    result
}

fn part_2(passports: &Vec<Vec<&str>>) -> u32 {
    let mut result = 0;

    let mandatory_fields: [(&str, fn(&str) -> bool); 7] = [
        ("byr", byr),
        ("iyr", iyr),
        ("eyr", eyr),
        ("hgt", hgt),
        ("hcl", hcl),
        ("ecl", ecl),
        ("pid", pid)
    ];

    'PASSPORTS_LOOP: for passport in passports {
        let keys_values: HashMap<String, String> = passport.iter().map(|x| {
            let parts: Vec<&str> = x.split(':').collect();
            (parts[0].to_string(), parts[1].to_string())
        }).collect();

        for field in mandatory_fields.iter() {
            match keys_values.get(field.0) {
                Some(value) => {
                    if !field.1(value) {
                        continue 'PASSPORTS_LOOP;
                    }
                }
                None => {
                    continue 'PASSPORTS_LOOP;
                }
            };
        }

        result += 1;
    }

    result
}

fn byr(data: &str) -> bool {
    match data.parse::<u32>() {
        Ok(year) => (1920..=2002).contains(&year),
        Err(_) => false,
    }
}

fn iyr(data: &str) -> bool {
    match data.parse::<u32>() {
        Ok(year) => (2010..=2020).contains(&year),
        Err(_) => false,
    }
}

fn eyr(data: &str) -> bool {
    match data.parse::<u32>() {
        Ok(year) => (2020..=2030).contains(&year),
        Err(_) => false,
    }
}

fn hgt(data: &str) -> bool {
    if let Some(value) = data.strip_suffix("cm") {
        match value.parse::<u32>() {
            Ok(height) => (150..=193).contains(&height),
            Err(_) => false,
        }
    } else if let Some(value) = data.strip_suffix("in") {
        match value.parse::<u32>() {
            Ok(height) => (59..=76).contains(&height),
            Err(_) => false,
        }
    } else {
        false
    }
}

fn hcl(data: &str) -> bool {
    let regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    regex.is_match(data)
}

fn ecl(data: &str) -> bool {
    matches!(data, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn pid(data: &str) -> bool {
    data.len() == 9 && data[..].chars().all(char::is_numeric)
}