fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

fn breakfast(input: &str) -> u32 {
    let mut values: Vec<u32> = input.chars()
        .into_iter()
        .map(|f| f.to_digit(10).unwrap())
        .collect();
    // add the first element to account for being circular
    values.push(values[0]);
    let windows: Vec<u32> = values.windows(2)
        .filter(|f| f[0] == f[1])
        .map(|f| f[0])
        .collect();
    let result_breakfast = windows.iter().sum();
    result_breakfast
}

fn lunch(input: &str) -> u32 {
    let mut values: Vec<u32> = input.chars()
        .into_iter()
        .map(|f| f.to_digit(10).unwrap())
        .collect();
    let part_one = &values[0..values.len()/2];
    let part_two = &values[values.len()/2..values.len()];
    let zipped = part_one.iter().zip(part_two);
    let filtered = zipped
        .filter(|f|f.0 == f.1)
        .map(|f|f.0);
    let result_lunch: u32 = filtered.into_iter().sum();
    result_lunch * 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 1102);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 1076);
    }
}
