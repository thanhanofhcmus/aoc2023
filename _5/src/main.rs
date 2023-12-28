use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct RangeMapping {
    source: usize,
    destination: usize,
    range: usize,
}

impl RangeMapping {
    fn forward(&self, n: usize) -> Option<usize> {
        if self.source <= n && n < self.source + self.range {
            return Some(self.destination + n - self.source);
        }
        None
    }

    fn backward(&self, n: usize) -> Option<usize> {
        if self.destination <= n && n < self.destination + self.range {
            return Some(self.source + n - self.destination);
        }
        None
    }

    fn bounds(&self) -> Vec<usize> {
        vec![
            self.source,
            self.source + self.range - 1,
            self.destination,
            self.destination + self.range - 1,
        ]
    }
}

struct LineMapping {
    range_mappings: Vec<RangeMapping>,
}

impl LineMapping {
    fn forward(&self, n: usize) -> usize {
        for rm in &self.range_mappings {
            if let Some(m) = rm.forward(n) {
                return m;
            }
        }
        n
    }

    fn backward(&self, n: usize) -> usize {
        for rm in &self.range_mappings {
            if let Some(m) = rm.backward(n) {
                return m;
            }
        }
        n
    }

    fn bounds(&self) -> Vec<usize> {
        self.range_mappings
            .iter()
            .flat_map(|rm| rm.bounds())
            .collect()
    }
}

struct PlaneMapping {
    line_mappings: Vec<LineMapping>,
}

impl PlaneMapping {
    fn forward(&self, n: usize) -> usize {
        self.line_mappings.iter().fold(n, |acc, lm| lm.forward(acc))
    }

    #[allow(dead_code)]
    fn backward(&self, n: usize) -> usize {
        self.line_mappings
            .iter()
            .fold(n, |acc, lm| lm.backward(acc))
    }

    fn start_bounds(&self) -> Vec<usize> {
        let mut sb: HashSet<usize> = HashSet::new();

        for (i, lm) in self.line_mappings.iter().enumerate() {
            let it = lm.bounds().into_iter().map(|b| {
                self.line_mappings[0..i]
                    .iter()
                    .rev()
                    .fold(b, |acc, lm| lm.backward(acc))
            });
            sb.extend(it);
        }

        sb.into_iter().collect()
    }
}

fn read_input(path: &str) -> Result<(Vec<usize>, PlaneMapping), &str> {
    let file_content = std::fs::read_to_string(path).map_err(|_| "error reading input file")?;
    let (seeds_input, mappings_input) = file_content
        .split_once("\n\n")
        .ok_or("error splitting seed and mapping")?;

    let seeds = seeds_input
        .split_once(':')
        .ok_or("parse seed: can not split ':'")?
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().map_err(|_| "error parsing seed"))
        .try_collect::<usize, Vec<usize>, &str>()?;

    let plane_mapping = PlaneMapping {
        line_mappings: mappings_input
            .split("\n\n")
            .map(|section| -> Result<LineMapping, &str> {
                Ok(LineMapping {
                    range_mappings: section
                        .lines()
                        .skip(1)
                        .map(|line| -> Result<RangeMapping, &str> {
                            let mut it = line.split_whitespace().map(str::parse::<usize>);
                            Ok(RangeMapping {
                                destination: it
                                    .next()
                                    .ok_or("error spliting to get destination")?
                                    .map_err(|_| "error parsing destination")?,
                                source: it
                                    .next()
                                    .ok_or("error spliting to get source")?
                                    .map_err(|_| "error parsing source")?,
                                range: it
                                    .next()
                                    .ok_or("error spliting to get range")?
                                    .map_err(|_| "error parsing range")?,
                            })
                        })
                        .try_collect()?,
                })
            })
            .try_collect()?,
    };

    Ok((seeds, plane_mapping))
}

#[allow(dead_code)]
fn part_1(path: &str) -> Result<usize, &str> {
    let (seeds, plane_mapping) = read_input(path)?;
    seeds
        .iter()
        .map(|&seed| plane_mapping.forward(seed))
        .min()
        .ok_or("error getting min after mapping")
}

#[allow(dead_code)]
fn part_2(path: &str) -> Result<usize, &str> {
    let (seeds, plane_mapping) = read_input(path)?;
    let ranges = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| -> Result<std::ops::Range<usize>, &str> {
            let start = *chunk.next().ok_or("error first element of chunk")?;
            let cnt = *chunk.next().ok_or("error second element of chunk")?;
            Ok(start..(start + cnt))
        })
        .try_collect::<std::ops::Range<usize>, Vec<std::ops::Range<usize>>, _>()?;

    let mut candidates = plane_mapping.start_bounds();
    candidates.append(&mut seeds.clone());

    // dbg!(ranges.clone());
    // dbg!(candidates.clone());

    candidates.retain(|c| ranges.iter().any(|r| r.contains(c)));

    // dbg!(candidates.clone());

    candidates
        .iter()
        .map(|&c| plane_mapping.forward(c))
        .min()
        .ok_or("error getting min")
}

fn main() {
    println!("{:?}", part_2("./input.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_part_1() {
        assert!(crate::part_1("./i2.txt") == Ok(35))
    }

    #[test]
    fn official_part_1() {
        assert!(crate::part_1("./input.txt") == Ok(178159714))
    }

    #[test]
    fn sample_part_2() {
        assert!(crate::part_2("./i2.txt") == Ok(46))
    }

    #[test]
    fn official_part_2() {
        assert!(crate::part_2("./input.txt") == Ok(100165128))
    }
}
