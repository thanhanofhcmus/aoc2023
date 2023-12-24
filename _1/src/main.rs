#![allow(dead_code)]

fn main() -> std::io::Result<()> {
    let s = std::fs::read_to_string("./input.txt")?;

    println!("{:?}", part_2(&s));

    Ok(())
}

fn part_1(s: &str) -> Option<usize> {
    s.trim()
        .lines()
        .map(|s| {
            (
                find_ascii_digit(s, str::find),
                find_ascii_digit(s, str::rfind),
            )
        })
        .map(|(l, r)| Some(l? * 10 + r?))
        .try_fold(0, |acc, n| Some(acc + n?))
}

fn part_2(s: &str) -> Option<usize> {
    s.trim()
        .lines()
        // .take(10)
        // .map(|s| {
        //     let v = get_value(s);
        //     println!("{} {:?}", s, &v);
        //     v
        // })
        .map(get_value)
        .try_fold(0, |acc, n| Some(acc + n?))
}

fn find_ascii_digit<'a, F>(s: &'a str, func: F) -> Option<usize>
where
    F: FnOnce(&'a str, fn(char) -> bool) -> Option<usize>,
{
    func(s, |c: char| c.is_ascii_digit())
        .and_then(|i| s.chars().nth(i))
        .map(|v| ((v as u8) - b'0') as usize)
}

fn get_value(s: &str) -> Option<usize> {
    let convert = |cap: regex::Captures<'_>| match *cap.extract::<1>().1.first()? {
        "eno" | "one" | "1" => Some(1),
        "owt" | "two" | "2" => Some(2),
        "eerht" | "three" | "3" => Some(3),
        "ruof" | "four" | "4" => Some(4),
        "evif" | "five" | "5" => Some(5),
        "xis" | "six" | "6" => Some(6),
        "neves" | "seven" | "7" => Some(7),
        "thgie" | "eight" | "8" => Some(8),
        "enin" | "nine" | "9" => Some(9),
        _ => None,
    };
    let re = regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let rev_re =
        regex::Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();

    let rev_s = s.chars().rev().collect::<String>();

    let l = convert(re.captures(s)?);
    let r = convert(rev_re.captures(&rev_s)?);

    Some(l? * 10 + r?)
}
