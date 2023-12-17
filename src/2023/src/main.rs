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

#[allow(dead_code)]
impl AoCC {
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
                        .map(|i| i as u32)
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
                        .map(|i| i as u32)
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

    let result = AoCC::day1(&buf);
    println!("{}", result);

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
}
