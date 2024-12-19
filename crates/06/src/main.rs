/// DOESNT WORK :(
use std::{collections::HashSet, fs};

fn find_inital_guard(columns: &Vec<&str>) -> Result<(usize, usize), String> {
    for (icol, col) in columns.iter().enumerate() {
        if col.contains("^") {
            let irow = col.find("^").unwrap();
            return Ok((icol, irow));
        }
    }
    Err(format!("Guard not found"))
}

#[derive(Debug)]
enum State {
    AtEnd,
    Found((usize, usize)),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn walk(rows: &Vec<&str>, pos: (usize, usize), direction: Direction) -> State {
    let (mut irow, mut icol) = pos;
    match direction {
        Direction::Up => {
            while irow > 0 {
                irow -= 1;
                if rows[irow].chars().nth(icol).unwrap() == '#' {
                    return State::Found((irow + 1, icol));
                }
            }
        }
        Direction::Down => {
            while irow < rows.len() - 1 {
                irow += 1;
                if rows[irow].chars().nth(icol).unwrap() == '#' {
                    return State::Found((irow - 1, icol));
                }
            }
        }
        Direction::Left => {
            while icol > 0 {
                icol -= 1;
                if rows[irow].chars().nth(icol).unwrap() == '#' {
                    return State::Found((irow, icol + 1));
                }
            }
        }
        Direction::Right => {
            while icol < rows[irow].len() - 1 {
                icol += 1;
                if rows[irow].chars().nth(icol).unwrap() == '#' {
                    return State::Found((irow, icol - 1));
                }
            }
        }
    }
    State::AtEnd
}

fn solution_a() {
    let contents = fs::read_to_string("crates/06/src/data.txt").unwrap();

    let rows: Vec<&str> = contents.lines().collect::<Vec<_>>();

    let mut current_pos = find_inital_guard(&rows).unwrap();

    let mut visited = HashSet::<(usize, usize)>::new();
    // visited.insert(current_pos);

    // we walk in the order: up, right, down, left
    // repeat recursively until we reach the end of the line
    let mut i = 0;
    loop {
        let direction = match i % 4 {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        };
        println!("Walking in direction: {:?}", direction);
        let state = walk(&rows, current_pos, direction.clone());
        match state {
            State::Found(pos) => {
                println!("Found obstacle at {:?}", pos);
                i += 1;

                // insert all positions between current_pos and pos into visited
                // keep in mind that there's an order
                match direction {
                    Direction::Up => {
                        for irow in pos.0..=current_pos.0 {
                            visited.insert((irow, current_pos.1));
                        }
                    }
                    Direction::Down => {
                        for irow in current_pos.0..=pos.0 {
                            visited.insert((irow, current_pos.1));
                        }
                    }
                    Direction::Left => {
                        for icol in pos.1..=current_pos.1 {
                            visited.insert((current_pos.0, icol));
                        }
                    }
                    Direction::Right => {
                        for icol in current_pos.1..=pos.1 {
                            visited.insert((current_pos.0, icol));
                        }
                    }
                }
                current_pos = pos;
            }
            State::AtEnd => {
                println!("End of line");
                break;
            }
        }
    }

    println!("Solution a: {:?}", visited.len());
}

fn solution_b() {
    println!("Solution b: {:?}", 0);
}

fn main() {
    solution_a();
    solution_b();
}
