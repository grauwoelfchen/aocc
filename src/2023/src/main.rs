use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use std::process::ExitCode;

mod day1;
mod day2;

static DATA_DIR: &str = "data";

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
        1 => day1::solution::part_two(&buf),
        2 => day2::solution::part_two(&buf),
        _ => unimplemented!(),
    };
    println!("day{}: {}", n, result);
    ExitCode::from(0)
}
