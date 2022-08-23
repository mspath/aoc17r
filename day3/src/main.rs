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

fn depth(cell: u32) -> u32 {
    // 0, 1,   2,   3,   4,   5, ..
    // 1, 3*3, 5*5, 7*7, 9*9, 11*11, ..
    let mut level = 0;
    if cell == 1 { return level; }

    loop {
        level = level + 1;
        let max = (level * 2 + 1) * (level * 2 + 1);
        if cell <= max { return level; }
    }
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

fn lunch(input: u32) -> u32 {
    input - input
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