use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;

static DATA_DIR: &str = "data";

#[allow(dead_code)]
impl AoC {
    pub fn day1(input: &str) -> u32 {
        input.lines().fold(0, |acc, dat| {
            let left = dat.chars().find_map(|c| c.to_digit(10));
            let right = dat.chars().rev().find_map(|c| c.to_digit(10));

            acc + match (left, right) {
                (Some(a), Some(b)) => a * 10 + b,
                _ => unreachable!(),
            }
        })
    }
}

pub struct AoC;

#[allow(dead_code)]
fn main() -> ExitCode {
    let path = Path::new(DATA_DIR).join("input.txt");
    if !path.exists() {
        eprintln!("no such file: {}", path.display());
        return ExitCode::from(1);
    }
    let mut f = File::open(path).expect("cannot open the file");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("cannot read the file");

    let result = AoC::day1(&buf);
    println!("{}", result);

    ExitCode::from(0)
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day1() {
        let result = AoC::day1("");
        assert_eq!(123, result);
    }
}
