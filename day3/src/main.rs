use std::collections::HashMap;

fn main() {
    let input = 368078;
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

enum Direction {
    East,
    North,
    West,
    South,
}

fn get_distance(cell: u32) -> u32 {
    let mut max_west = 0;
    let mut max_east = 0;
    let mut max_north = 0;
    let mut max_south = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = Direction::East;

    for _ in 1..cell {
        match direction {
            Direction::East => {
                x = x + 1;
                if x > max_east {
                    max_east = x;
                    direction = Direction::North;
                }
            }
            Direction::North => {
                y = y - 1;
                if y < max_north {
                    max_north = y;
                    direction = Direction::West;
                }
            }
            Direction::West => {
                x = x - 1;
                if x < max_west {
                    max_west = x;
                    direction = Direction::South;
                }  
            }
            Direction::South => {
                y = y + 1;
                if y > max_south {
                    max_south = y;
                    direction = Direction::East;
                }
            }
        }
    }
    let d = x.abs() + y.abs();
    d.try_into().unwrap()
}

fn breakfast(input: u32) -> u32 {
    let distance = get_distance(input);
    distance
}

fn get_stress_value(input: u32) -> u32 {
    let mut grid = HashMap::<(i32, i32), u32>::new();
    grid.insert((0, 0), 1);

    let mut max_west = 0;
    let mut max_east = 0;
    let mut max_north = 0;
    let mut max_south = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = Direction::East;

    loop {
        match direction {
            Direction::East => {
                x = x + 1;
                if x > max_east {
                    max_east = x;
                    direction = Direction::North;
                }
            }
            Direction::North => {
                y = y - 1;
                if y < max_north {
                    max_north = y;
                    direction = Direction::West;
                }
            }
            Direction::West => {
                x = x - 1;
                if x < max_west {
                    max_west = x;
                    direction = Direction::South;
                }  
            }
            Direction::South => {
                y = y + 1;
                if y > max_south {
                    max_south = y;
                    direction = Direction::East;
                }
            }
        }
        let nw = grid.get(&(x - 1, y - 1)).cloned().unwrap_or(0);
        let n = grid.get(&(x, y - 1)).cloned().unwrap_or(0);
        let ne = grid.get(&(x + 1, y - 1)).cloned().unwrap_or(0);
        let w = grid.get(&(x - 1, y)).cloned().unwrap_or(0);
        let e = grid.get(&(x + 1, y)).cloned().unwrap_or(0);
        let sw = grid.get(&(x - 1, y + 1)).cloned().unwrap_or(0);
        let s = grid.get(&(x, y + 1)).cloned().unwrap_or(0);
        let se = grid.get(&(x + 1, y + 1)).cloned().unwrap_or(0);
        let grid_value = nw + n + ne + w + e + sw + s + se;
        grid.insert((x, y), grid_value);

        // result is the first value larger than input
        if grid_value > input { return grid_value; }
    }
}

fn lunch(input: u32) -> u32 {
    get_stress_value(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(368078), 371);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(368078), 369601);
    }
}