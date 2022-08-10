fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

fn breakfast(input: &str) -> u32 {
    let spreadsheet: Vec<Vec<u32>> = input.split("\n").into_iter().map(|row| {
        let values: Vec<u32> = row.split_whitespace()
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        values
    }).collect();

    let checksum: u32 = spreadsheet.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum();

    checksum
}

// this will return the first match as per instructions 
fn checksum_vec(values: Vec<u32>) -> u32 {
    for i in &values {
        for j in &values {
            if i % j == 0 && i != j {
                return i / j;
            }
        }
    }
    0
}

fn lunch(input: &str) -> u32 {
    let spreadsheet: Vec<Vec<u32>> = input.split("\n").into_iter().map(|row| {
        let values: Vec<u32> = row.split_whitespace()
            .into_iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        values
    }).collect();

    let checksum = spreadsheet.into_iter()
        .map(|row| checksum_vec(row))
        .sum();
    checksum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 42299);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 277);
    }
}