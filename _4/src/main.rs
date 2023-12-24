use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let result = part_1(&input);

    println!("{:?}", result);
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| count_(split_card(s).unwrap()))
        .filter(|&x| x > 0)
        .map(|x| 2_u32.pow((x as u32) - 1))
        .sum()
}

fn split_card(input: &str) -> Option<(&str, &str)> {
    let idx = input.find(": ")?;
    input[idx..]
        .split_once(" | ")
        .map(|(s1, s2)| (s1.trim(), s2.trim()))
}

fn count_((s1, s2): (&str, &str)) -> usize {
    let h1 = s1.split_whitespace().collect::<HashSet<&str>>();
    let h2 = s2.split_whitespace().collect::<HashSet<&str>>();

    let c = h1.intersection(&h2).count();

    c
}
