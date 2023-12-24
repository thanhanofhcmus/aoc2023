fn main() -> std::io::Result<()> {
    let s = std::fs::read_to_string("./input.txt")?;

    let lines = s.trim().lines().collect::<Vec<&str>>();

    let r = lines
        .into_iter()
        .map(|s| {
            (
                find_ascii_digit(s, str::find),
                find_ascii_digit(s, str::rfind),
            )
        })
        .map(|(l, r)| Some(l? * 10 + r?))
        .fold(Some(0), |acc, n| Some(acc? + n?));

    println!("{:?}", r);

    Ok(())
}

fn find_ascii_digit<'a, F>(s: &'a str, func: F) -> Option<usize>
where
    F: FnOnce(&'a str, fn(char) -> bool) -> Option<usize>,
{
    func(s, |c: char| c.is_ascii_digit())
        .and_then(|i| s.chars().nth(i))
        .map(|v| ((v as u8) - b'0') as usize)
}
