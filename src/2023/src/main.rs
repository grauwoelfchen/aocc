use std::collections::HashMap;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;

static DATA_DIR: &str = "data";

// NOTE: index matches its digit value
static DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine",
];

static CUBES: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

#[allow(dead_code)]
impl AoCC {
    fn line_to_cube_map(line: &str) -> HashMap<&str, u32> {
        let (_, games) = line.split_once(':').unwrap();
        let cubes: Vec<&str> = games.split(|c| c == ';' || c == ',').collect();
        let mut cubes_pair: Vec<(&str, u32)> = cubes
            .iter()
            .map(|s| {
                // eg. " 3 red"
                let mut exp = s.split(' ');
                let num = (exp.nth(1).unwrap()).parse::<u32>().unwrap();
                let key = exp.last().unwrap();
                (key, num)
            })
            .collect();

        // last values are used in HashMap
        cubes_pair.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        cubes_pair.iter().cloned().collect()
    }

    pub fn day2_part_one(input: &str) -> u32 {
        let map: HashMap<&str, u32> = CUBES.iter().cloned().collect();

        input.lines().enumerate().fold(0, |mut acc, (i, line)| {
            let data = Self::line_to_cube_map(line);

            if data.iter().all(|(k, v)| &map[k] >= v) {
                acc += (i + 1) as u32;
            }
            acc
        })
    }

    pub fn day2_part_two(input: &str) -> u32 {
        input.lines().fold(0, |acc, line| {
            let data = Self::line_to_cube_map(line);

            acc + data.values().copied().reduce(|a, b| a * b).unwrap()
        })
    }

    pub fn day1(input: &str) -> u32 {
        input.lines().fold(0, |acc, line| {
            let mut i_c = line.char_indices();

            // NOTE:
            // * This doesn't work for "twenty" (20) or "thirteen" (13) etc.
            let left = i_c.find_map(|(i, c)| {
                c.to_digit(10).or_else(|| {
                    DIGITS
                        .iter()
                        .position(|d| line[0..=i].contains(d))
                        .map(|p| p as u32)
                })
            });

            // NOTE:
            // * eg. "seventeen" (17) can be treated as 7
            // * eg. "foury" means 0
            // * eg. "fourtyone" (41) can be treated as 1
            let right = i_c.rev().find_map(|(i, c)| {
                c.to_digit(10).or_else(|| {
                    DIGITS
                        .iter()
                        .position(|d| line[i..].contains(d))
                        .map(|p| p as u32)
                })
            });

            acc + match (left, right) {
                (Some(a), Some(b)) => a * 10 + b,
                (Some(a), None) => a * 10 + a,
                _ => unreachable!(),
            }
        })
    }
}

pub struct AoCC;

#[allow(dead_code)]
fn main() -> ExitCode {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("need a day number as an only one argument");
        return ExitCode::from(1);
    }
    let dayn = args.nth(1).unwrap();
    let n = dayn
        .replace("day", "")
        .parse::<u8>()
        .expect("must be a value that can be parsed as a number");
    if !(1..26).contains(&n) {
        eprintln!("the day number must be in a range of 1..=25");
        return ExitCode::from(1);
    }

    let path = Path::new(DATA_DIR).join(format!("input-day{}.txt", n));
    if !path.exists() {
        eprintln!("no such file: {}", path.display());
        return ExitCode::from(1);
    }
    let mut f = File::open(path).expect("cannot open the file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("cannot read the file");

    let result = match n {
        1 => AoCC::day1(&buf),
        2 => AoCC::day2_part_two(&buf),
        _ => unimplemented!(),
    };
    println!("day{}: {}", n, result);
    ExitCode::from(0)
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        let result = AoCC::day1(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#,
        );
        assert_eq!(142, result);

        let result = AoCC::day1(
            r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#,
        );
        assert_eq!(281, result);
    }

    #[test]
    fn test_day2_part_one() {
        let result = AoCC::day2_part_one(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#,
        );
        assert_eq!(8, result);
    }

    #[test]
    fn test_day2_part_two() {
        let result = AoCC::day2_part_two(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#,
        );
        assert_eq!(2286, result);
    }
}
