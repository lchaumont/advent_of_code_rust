use std::collections::HashMap;

type Position = (usize, usize);

#[derive(Debug, Clone)]
enum CellState {
    FLOOR,
    EMPTY,
    OCCUPIED,
}
#[derive(Debug)]
struct Board {
    cells: HashMap<Position, CellState>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            cells: HashMap::new(),
            width,
            height,
        }
    }

    fn count_occupied_neighbors_at_position(&self, (target_x, target_y): Position) -> usize {
        let possible_deltas: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        possible_deltas.iter()
            .map(|(dx, dy)| (target_x as isize + dx, target_y as isize + dy))
            .filter_map(|(x, y)| self.cells.get(&(x as usize, y as usize)))
            .filter(|cell| match cell {
                CellState::OCCUPIED => true,
                _ => false
            })
            .count()
    }

    fn count_visible_at_position(&self, position: Position) -> usize {
        let directions: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

        let mut count: usize = 0;

        for direction in directions.iter() {
            count += self.count_visible_at_position_in_direction(position, *direction);
        }

        count
    }

    fn count_visible_at_position_in_direction(&self, (target_x, target_y): Position, (dir_x, dir_y): (isize, isize)) -> usize {
        let mut x = target_x as isize + dir_x;
        let mut y = target_y as isize + dir_y;

        while x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            match self.cells.get(&(x as usize, y as usize)) {
                Some(CellState::OCCUPIED) => return 1,
                Some(CellState::EMPTY) => return 0,
                _ => {}
            }

            x += dir_x;
            y += dir_y;
        }

        0
    }
}

pub fn run(input: String) {
    let mut cells: HashMap<Position, CellState> = HashMap::new();

    for (line_index, line) in input.trim_end().split("\r\n").enumerate() {
        for (column_index, char) in line.chars().enumerate() {
            let cell = match char {
                '.' => CellState::FLOOR,
                'L' => CellState::EMPTY,
                '#' => CellState::OCCUPIED,
                _ => panic!("Unexpected character {}", char),
            };

            cells.insert((line_index, column_index), cell);
        }
    }

    let width = cells.keys().map(|(x, _)| x).max().unwrap() + 1;
    let height = cells.keys().map(|(_, y)| y).max().unwrap() + 1;

    let board = Board {
        cells,
        width,
        height,
    };

    //println!("Part 1: {:?}", part_1(board));
    println!("Part 2: {:?}", part_2(board));
}

fn iteration_part1(board: &Board) -> (Board, bool) {
    let mut next_board = Board::new(board.width, board.height);
    let mut changed = false;

    for (position, cell) in board.cells.iter() {
        match cell {
            CellState::EMPTY => {
                let occupied_neighbors = board.count_occupied_neighbors_at_position(*position);
                if occupied_neighbors == 0 {
                    next_board.cells.insert(*position, CellState::OCCUPIED);
                    changed = true;
                } else {
                    next_board.cells.insert(*position, CellState::EMPTY);
                }
            }
            CellState::OCCUPIED => {
                let occupied_neighbors = board.count_occupied_neighbors_at_position(*position);
                if occupied_neighbors >= 4 {
                    next_board.cells.insert(*position, CellState::EMPTY);
                    changed = true;
                } else {
                    next_board.cells.insert(*position, CellState::OCCUPIED);
                }
            }
            CellState::FLOOR => {
                next_board.cells.insert(*position, CellState::FLOOR);
            }
        }
    }

    (next_board, changed)
}

fn part_1(initial_board: Board) -> isize {
    let mut current_board = initial_board;

    loop {
        let (next_board, changed) = iteration_part1(&current_board);
        if !changed {
            break;
        }

        current_board = next_board;
    }

    current_board.cells.values().filter(|cell| match cell {
        CellState::OCCUPIED => true,
        _ => false
    }).count() as isize
}

fn iteration_part2(board: &Board) -> (Board, bool) {
    let mut next_board = Board::new(board.width, board.height);
    let mut changed = false;

    for (position, cell) in board.cells.iter() {
        match cell {
            CellState::EMPTY => {
                let occupied_neighbors = board.count_visible_at_position(*position);
                if occupied_neighbors == 0 {
                    next_board.cells.insert(*position, CellState::OCCUPIED);
                    changed = true;
                } else {
                    next_board.cells.insert(*position, CellState::EMPTY);
                }
            }
            CellState::OCCUPIED => {
                let occupied_neighbors = board.count_visible_at_position(*position);
                if occupied_neighbors >= 5 {
                    next_board.cells.insert(*position, CellState::EMPTY);
                    changed = true;
                } else {
                    next_board.cells.insert(*position, CellState::OCCUPIED);
                }
            }
            CellState::FLOOR => {
                next_board.cells.insert(*position, CellState::FLOOR);
            }
        }
    }

    (next_board, changed)
}

fn part_2(initial_board: Board) -> isize {
    let mut current_board = initial_board;

    loop {
        let (next_board, changed) = iteration_part2(&current_board);
        if !changed {
            break;
        }

        current_board = next_board;
    }

    current_board.cells.values().filter(|cell| match cell {
        CellState::OCCUPIED => true,
        _ => false
    }).count() as isize
}