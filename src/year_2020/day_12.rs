#[derive(Debug)]
enum Direction {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

#[derive(Debug)]
enum Orientation {
    N,
    S,
    E,
    W
}

fn turn_clockwise(current: Orientation) -> Orientation {
    match current {
        Orientation::N => Orientation::E,
        Orientation::S => Orientation::W,
        Orientation::E => Orientation::S,
        Orientation::W => Orientation::N
    }
}

fn turn_counter_clockwise(current: Orientation) -> Orientation {
    match current {
        Orientation::N => Orientation::W,
        Orientation::S => Orientation::E,
        Orientation::E => Orientation::N,
        Orientation::W => Orientation::S
    }
}

pub fn run(input: String) {
    let directions: Vec<Direction> = input.lines().map(|x| {
        let value = x[1..].parse::<i64>().unwrap();
        match &x[0..1] {
            "N" => Direction::N(value),
            "S" => Direction::S(value),
            "E" => Direction::E(value),
            "W" => Direction::W(value),
            "L" => Direction::L(value),
            "R" => Direction::R(value),
            "F" => Direction::F(value),
            _ => panic!("Unexpected value: {}", &x[0..1])
        }
    }).collect();

    println!("Part 1: {:?}", part_1(&directions));
    println!("Part 2: {:?}", part_2(&directions));
}

fn part_1(directions: &Vec<Direction>) -> i64 {
    let mut current_position: (i64, i64) = (0, 0);
    let mut current_orientation = Orientation::E;

    for direction in directions {
        match direction {
            Direction::N(value) => {
                current_position.1 += value;
            }
            Direction::S(value) => {
                current_position.1 -= value;
            }
            Direction::E(value) => {
                current_position.0 += value;
            }
            Direction::W(value) => {
                current_position.0 -= value;
            }
            Direction::L(value) => {
                let times = value / 90;
                for _ in 0..times {
                    current_orientation = turn_counter_clockwise(current_orientation);
                }
            }
            Direction::R(value) => {
                let times = value / 90;
                for _ in 0..times {
                    current_orientation = turn_clockwise(current_orientation);
                }
            }
            Direction::F(value) => {
                match current_orientation {
                    Orientation::N => {
                        current_position.1 += value;
                    }
                    Orientation::S => {
                        current_position.1 -= value;
                    }
                    Orientation::E => {
                        current_position.0 += value;
                    }
                    Orientation::W => {
                        current_position.0 -= value;
                    }
                }
            }
        }
    }

    current_position.0.abs() + current_position.1.abs()
}

fn part_2(directions: &Vec<Direction>) -> i64 {
    let mut ship_position: (i64, i64) = (0, 0);
    let mut waypoint_position: (i64, i64) = (10, 1);

    for direction in directions {
        match direction {
            Direction::N(value) => {
                waypoint_position.1 += value;
            }
            Direction::S(value) => {
                waypoint_position.1 -= value;
            }
            Direction::E(value) => {
                waypoint_position.0 += value;
            }
            Direction::W(value) => {
                waypoint_position.0 -= value;
            }
            Direction::L(value) => {
                let times = value / 90;
                for _ in 0..times {
                    let y = waypoint_position.1;
                    waypoint_position.1 = waypoint_position.0;
                    waypoint_position.0 = -y;
                }
            }
            Direction::R(value) => {
                let times = value / 90;
                for _ in 0..times {
                    let y = waypoint_position.1;
                    waypoint_position.1 = -waypoint_position.0;
                    waypoint_position.0 = y;
                }
            }
            Direction::F(value) => {
                ship_position.0 += value * waypoint_position.0;
                ship_position.1 += value * waypoint_position.1;
            }
        }
    }

    ship_position.0.abs() + ship_position.1.abs()
}