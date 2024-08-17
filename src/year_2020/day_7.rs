use std::collections::HashSet;
use regex::Regex;

pub fn run(input: String) {
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let mut set_of_bags: HashSet<String> = HashSet::new();
    get_bags_that_can_contains_bag(input, "shiny gold", &mut set_of_bags);

    println!("Part 1: {:?}", set_of_bags.len());
}

fn part_2(input: &String) {
    let count: u64 = get_bags_that_bag_is_contained(input, "shiny gold");
    println!("Part 2: {:?}", count);
}

fn get_bags_that_can_contains_bag(input: &String, current_bag: &str, set_of_bags: &mut HashSet<String>) {
    let mut new_bags = HashSet::new();

    for line in input.lines() {
        if line.contains(current_bag) {
            let key_bag = extract_key_bag_from_line(line);
            if &key_bag != current_bag && !set_of_bags.contains(&key_bag) {
                new_bags.insert(key_bag);
            }
        }
    }

    for bag in new_bags {
        set_of_bags.insert(bag.clone());
        get_bags_that_can_contains_bag(input, &bag, set_of_bags);
    }
}

fn extract_key_bag_from_line(line: &str) -> String {
    let splitted = line.split(" ").collect::<Vec<&str>>();
    format!("{} {}", splitted.get(0).unwrap(), splitted.get(1).unwrap())
}

fn extract_bags_from_line(line: &str) -> Vec<String> {
    let re = Regex::new(r"(\d) ([a-z]+ [a-z]+)").unwrap();
    let mut bags = Vec::new();

    for cap in re.captures_iter(line) {
        bags.push(format!("{} {}", &cap[1], &cap[2]));
    }

    bags
}

fn get_bags_that_bag_is_contained(input: &String, current_bag: &str) -> u64 {
    for line in input.lines() {
        if extract_key_bag_from_line(line) == current_bag {
            let bags = extract_bags_from_line(line);

            if !bags.is_empty() {
                return bags.iter().map(|bag| {
                    let parts: Vec<&str> = bag.split_whitespace().collect();
                    let number = parts[0].parse::<u64>().unwrap();
                    let bag_name = format!("{} {}", parts[1], parts[2]);
                    number + number * get_bags_that_bag_is_contained(input, &bag_name)
                }).sum();
            }

            return 0;
        }
    }

    eprintln!("Bag not found: {}", current_bag);
    0
}