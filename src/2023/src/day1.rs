#[allow(dead_code)]
pub mod solution {
    // NOTE: index matches its digit value
    static DIGITS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine",
    ];

    fn add_calibration_values(left: Option<u32>, right: Option<u32>) -> u32 {
        match (left, right) {
            (Some(a), Some(b)) => a * 10 + b,
            (Some(a), None) => a * 10 + a,
            _ => unreachable!(),
        }
    }

    pub fn part_one(input: &str) -> u32 {
        input.lines().fold(0, |acc, line| {
            let mut chars = line.chars();

            let left = chars.find_map(|c| c.to_digit(10));

            let right = chars.rev().find_map(|c| c.to_digit(10));

            acc + add_calibration_values(left, right)
        })
    }

    pub fn part_two(input: &str) -> u32 {
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

            acc + add_calibration_values(left, right)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solution::part_one(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#,
        );
        assert_eq!(142, result);
    }

    #[test]
    fn test_part_two() {
        let result = solution::part_two(
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
