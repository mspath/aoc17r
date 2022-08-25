const RANGE: usize = 1058;

fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

fn vec_into_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn breakfast(input: &str) -> usize {
    let numbers: Vec<i32> = input
        .lines()
        .into_iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    let mut instructions: [i32; RANGE] = vec_into_array(numbers);
    let mut counter: usize = 0;
    let mut index: usize = 0;
    loop {
        counter += 1;
        let step: i32 = instructions[index];
        let next: i32 = step + index as i32;
        if next < 0 || next > RANGE as i32 { return counter; }
        instructions[index] += 1;
        index = add(index, step).unwrap();
        if index >= RANGE { return counter; }
    }
}

fn lunch(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 356945);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 28372145);
    }
}