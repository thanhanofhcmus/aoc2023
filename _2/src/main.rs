#[derive(Default, Debug)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}

impl Bag {
    fn merge_max(&self, rhs: &Self) -> Self {
        Self {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}

impl Bag {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Bag { red, green, blue }
    }
}

#[derive(Default)]
struct Game {
    id: usize,
    bag: Bag,
}

impl Game {
    fn new(id: usize, bag: Bag) -> Self {
        Self { id, bag }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let result = part_2(&input);

    println!("{result}");
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(parse_game)
        .filter_map(|g| {
            if g.bag.red <= 12 && g.bag.green <= 13 && g.bag.blue <= 14 {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter_map(parse_game)
        .map(|g| g.bag.red * g.bag.blue * g.bag.green)
        .sum()
}

fn parse_game(input: &str) -> Option<Game> {
    let parts = input.split(':').collect::<Vec<&str>>();

    // dbg!(&parts);

    let id = parse_last_number(parts.first()?)?;
    let bag = parse_bag(parts.get(1)?)?;

    Some(Game::new(id, bag))
}

fn parse_last_number(input: &str) -> Option<usize> {
    input.trim().split(' ').last()?.parse::<usize>().ok()
}

fn parse_bag(input: &str) -> Option<Bag> {
    input
        .trim()
        .split(';')
        .map(parse_bag_set)
        .try_fold(Bag::default(), |acc, el| Some(acc.merge_max(&el?)))
}

fn parse_bag_set(input: &str) -> Option<Bag> {
    input
        .trim()
        .split(',')
        .map(parse_bag_part)
        .try_fold(Bag::default(), |acc, el| Some(acc.merge_max(&el?)))
}

fn parse_bag_part(input: &str) -> Option<Bag> {
    let mut it = input.trim().split(' ');
    let value = it.next()?.parse::<usize>().ok()?;
    let name = it.last()?;

    let bag = match name {
        "red" => Bag::new(value, 0, 0),
        "green" => Bag::new(0, value, 0),
        "blue" => Bag::new(0, 0, value),
        _ => return None,
    };

    Some(bag)
}
