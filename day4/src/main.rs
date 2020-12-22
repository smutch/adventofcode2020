use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn get_year(key: &str, entry: &mut HashMap<&str, &str>) -> u32 {
    entry
        .get(key)
        .unwrap_or(&"")
        .parse::<u32>()
        .ok()
        .unwrap_or_default()
}

fn validate_and_clear(entry: &mut HashMap<&str, &str>) -> u32 {
    let byr = get_year("byr", entry);
    let iyr = get_year("iyr", entry);
    let eyr = get_year("eyr", entry);

    if !((entry.len() == 8 || (entry.len() == 7 && !entry.contains_key("cid")))
        && (byr >= 1920 && byr <= 2002)
        && (iyr >= 2010 && iyr <= 2020)
        && (eyr >= 2020 && eyr <= 2030))
    {
        entry.clear();
        return 0;
    }

    let hgt_str = entry.get(&"hgt").unwrap_or(&"");
    if hgt_str.len() < 3 {
        return 0;
    } else {
        let (hgt_str, unit) = hgt_str.split_at(hgt_str.len() - 2);
        let hgt = hgt_str.parse::<u32>().ok().unwrap_or_default();
        if !match unit {
            "cm" => hgt >= 150 && hgt <= 193,
            "in" => hgt >= 59 && hgt <= 76,
            &_ => false,
        } {
            entry.clear();
            return 0;
        }
    }
    let hcl = entry.get(&"hcl").unwrap_or(&"");
    if !((hcl.len() == 7)
        && hcl.starts_with('#')
        && hcl
            .get(1..)
            .unwrap()
            .chars()
            .all(|c| c.is_numeric() || matches!(c, 'a'..='f')))
    {
        entry.clear();
        return 0;
    }
    let ecl = entry.get(&"ecl").unwrap_or(&"");
    if !match ecl {
        &"amb" | &"blu" | &"brn" | &"gry" | &"grn" | &"hzl" | &"oth" => true,
        &_ => false,
    } {
        entry.clear();
        return 0;
    };

    let pid = entry.get(&"pid").unwrap_or(&"");
    if !(pid.len() == 9 && pid.chars().all(|c| c.is_numeric())) {
        entry.clear();
        return 0;
    };

    entry.clear();
    1
}

fn main() {
    let args = Cli::from_args();
    let input = std::fs::read_to_string(args.input).expect("Failed to read input file.");
    let mut entry = HashMap::new();
    let mut valid_count = 0;
    for line in input.lines() {
        if line.trim_end().is_empty() {
            valid_count += validate_and_clear(&mut entry);
        } else {
            for part in line.split_whitespace() {
                let kv: Vec<&str> = part.splitn(2, ':').collect();
                entry.insert(kv[0], kv[1]);
            }
        }
    }
    valid_count += validate_and_clear(&mut entry);

    println!("Valid entries = {}", valid_count);
}
