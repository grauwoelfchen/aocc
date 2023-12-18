#[allow(dead_code)]
pub mod solution {
    use std::collections::HashMap;

    static CUBES: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

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

    pub fn part_one(input: &str) -> u32 {
        let map: HashMap<&str, u32> = CUBES.iter().cloned().collect();

        input.lines().enumerate().fold(0, |mut acc, (i, line)| {
            let data = line_to_cube_map(line);

            if data.iter().all(|(k, v)| &map[k] >= v) {
                acc += (i + 1) as u32;
            }
            acc
        })
    }

    pub fn part_two(input: &str) -> u32 {
        input.lines().fold(0, |acc, line| {
            let data = line_to_cube_map(line);

            acc + data.values().copied().reduce(|a, b| a * b).unwrap()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solution::part_one(
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
    fn test_part_two() {
        let result = solution::part_two(
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
