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

    let result = walk(&mut input);

    println!("{result}");

    Ok(())
}

fn walk(input: &mut Vec<Vec<char>>) -> usize {
    let n = input.len();
    let m = input[0].len();
    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            let c = input[i][j];
            if c == '.' || c.is_ascii_digit() {
                continue;
            }

            for (dx, dy) in &D {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                    continue;
                }
                result += process(input, x as usize, y as usize);
            }
        }
    }

    result
}

fn process(input: &mut Vec<Vec<char>>, x: usize, y: usize) -> usize {
    if !input[x][y].is_ascii_digit() {
        return 0;
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

    for i in left..right + 1 {
        input[x][i] = '.'
    }

    println!("{}", r);

    r
}

fn char_slice_to_usize(cs: &[char]) -> usize {
    let mut r = 0;
    for &c in cs {
        r = r * 10 + ((c as u8) - b'0') as usize
    }
    r
}
