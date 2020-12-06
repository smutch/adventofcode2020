use std::convert::TryInto;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Entry {
    fn parse(line: &str) -> Entry {
        let parts: Vec<_> = line.splitn(3, ' ').collect();
        let range: Vec<_> = parts[0]
            .split('-')
            .map(|v| v.parse::<usize>().expect("Failed to parse range."))
            .collect();
        let letter = String::from(parts[1])
            .strip_suffix(':')
            .expect("failed to strip :")
            .parse::<char>()
            .expect("Failed to parse letter");
        let password = String::from(parts[2]);
        Entry {
            min: range[0],
            max: range[1],
            letter: letter,
            password: password,
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let input: Vec<_> = std::fs::read_to_string(args.input)
        .expect("Failed to read input file.")
        .lines()
        .map(|l| Entry::parse(l))
        .collect();

    // Part 1 {{{
    println!("=== Part 1 ===");
    let mut valid: u32 = 0;
    let mut invalid: u32 = 0;
    for entry in &input {
        let count = entry
            .password
            .chars()
            .filter(|&c| c == entry.letter)
            .count();
        if (entry.min <= count) && (count <= entry.max) {
            valid += 1;
        } else {
            invalid += 1;
        }
    }

    assert_eq!(valid + invalid, input.len().try_into().unwrap());

    println!("N valid passwords = {}(/{})", valid, input.len());
    println!("==============\n");
    // }}}

    // Part 2 {{{
    println!("=== Part 2 ===");
    let valid = (&input)
        .into_iter()
        .filter(|entry| {
            (entry.password.chars().nth(entry.min-1).unwrap() == entry.letter)
                ^ (entry.password.chars().nth(entry.max-1).unwrap() == entry.letter)
        })
        .count();

    println!("N valid passwords = {:?}(/{})", valid, input.len());
    println!("==============\n");
    // }}}
}
