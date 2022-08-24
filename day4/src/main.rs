use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result_breakfast = breakfast(input);
    println!("breakfast: {}", result_breakfast);

    let result_lunch = lunch(input);
    println!("lunch: {}", result_lunch);
}

fn valid_passphrase(passphrase: &str) -> bool {
    let size = passphrase.split(" ").count();
    let uniques: HashSet<&str> = passphrase.split(" ").into_iter().collect();
    uniques.len() == size
}

fn breakfast(input: &str) -> usize {
    let passwords: Vec<&str> = input.split("\n").into_iter().collect();
    let valid_passwords: Vec<&str> = passwords.into_iter().filter(|passphrase| valid_passphrase(passphrase)).into_iter().collect();
    valid_passwords.len()
}

fn sort_chars(password: &str) -> Vec<char> {
    let mut letters = password.chars().collect::<Vec<_>>();
    letters.sort_unstable();
    letters
}

fn secure_passphrase(passphrase: &str) -> bool {
    let size = passphrase.split(" ").count();
    let uniques: HashSet<Vec<char>> = passphrase
        .split(" ")
        .map(|password| sort_chars(password))
        .into_iter()
        .collect();
    uniques.len() == size
}

fn lunch(input: &str) -> usize {
    let passwords: Vec<&str> = input.split("\n").into_iter().collect();
    let secure_passwords: Vec<&str> = passwords
        .into_iter()
        .filter(|passphrase| secure_passphrase(passphrase))
        .into_iter()
        .collect();
    secure_passwords.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_breakfast() {
        assert_eq!(super::breakfast(include_str!("input.txt")), 383);
    }

    #[test]
    fn test_lunch() {
        assert_eq!(super::lunch(include_str!("input.txt")), 265);
    }
}