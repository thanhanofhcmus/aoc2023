#![allow(dead_code)]

const D: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() -> std::io::Result<()> {
    let mut input = std::fs::read_to_string("./input.txt")?
        // let mut input = std::fs::read_to_string("./i2.txt")?
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // let result = walk(&mut input, part_1);
    let result = walk(&mut input, part_2);

    println!("{result}");

    Ok(())
}

fn walk(
    input: &mut Vec<Vec<char>>,
    call_fn: fn(input: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize,
) -> usize {
    let n = input.len();
    let m = input[0].len();
    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            let c = input[i][j];
            if c == '.' || c.is_ascii_digit() {
                continue;
            }
            result += call_fn(input, i, j);
        }
    }

    result
}

fn part_1(input: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut result = 0;
    for (dx, dy) in &D {
        let (u, v) = (x as i32 + dx, y as i32 + dy);
        if u < 0 || v < 0 || u >= input.len() as i32 || v >= input[0].len() as i32 {
            continue;
        }

        let (r, left, right) = process(input, u as usize, v as usize);

        result += r;

        if r == 0 {
            continue;
        }

        for z in left..right + 1 {
            input[u as usize][z] = '.'
        }
    }

    result
}

fn part_2(input: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if input[x][y] != '*' {
        return 0;
    }

    let mut posed = std::collections::HashMap::new();

    for (dx, dy) in &D {
        let (u, v) = (x as i32 + dx, y as i32 + dy);
        if u < 0 || v < 0 || u >= input.len() as i32 || v >= input[0].len() as i32 {
            continue;
        }

        let (r, left, _) = process(input, u as usize, v as usize);

        if r == 0 {
            continue;
        }

        posed.insert((left, u), r);
    }

    if posed.len() != 2 {
        return 0;
    }

    posed.values().product()
}

fn process(input: &[Vec<char>], x: usize, y: usize) -> (usize, usize, usize) {
    if !input[x][y].is_ascii_digit() {
        return (0, 0, 0);
    }

    let left = (|| {
        for i in (0..y).rev() {
            if !input[x].get(i).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                return i + 1;
            }
        }
        0
    })();

    let right = (|| {
        for i in y.. {
            if !input[x].get(i).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                return i - 1;
            }
        }
        input[x].len() - 1
    })();

    let r = char_slice_to_usize(&input[x][left..right + 1]);
    (r, left, right)
}

fn char_slice_to_usize(cs: &[char]) -> usize {
    let mut r = 0;
    for &c in cs {
        r = r * 10 + ((c as u8) - b'0') as usize
    }
    r
}
