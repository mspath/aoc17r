fn main() {
    let input = include_str!("input.txt");
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
    let result_breakfast: u32 = windows.iter().sum();
    println!("{}", result_breakfast);
    // drop the first element again for part2
    values.pop();
    let part_one = &values[0..values.len()/2];
    let part_two = &values[values.len()/2..values.len()];
    let zipped = part_one.iter().zip(part_two);
    let filtered = zipped
        .filter(|f|f.0 == f.1)
        .map(|f|f.0);
    let result_lunch: u32 = filtered.into_iter().sum();
    println!("{}", result_lunch * 2);
}
