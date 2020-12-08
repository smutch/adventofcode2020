use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let input = std::fs::read_to_string(args.input).expect("Failed to read input file.");

    // let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let mut pos = (0usize, 0usize);
    let movement = (3i16, 1i16);

    let ntrees = input
        .lines()
        .step_by(movement.1 as usize)
        .fold(0, |count, line| {
            println!("{},{} -> {}", pos.0, pos.1, line.chars().nth(pos.0).unwrap());
            let mut new_count = count;
            if line.chars().nth(pos.0).unwrap() == '#' {
                new_count += 1;
            }
            pos.0 = (pos.0 + movement.0 as usize) % width;
            pos.1 += movement.1 as usize;
            new_count
        });

    println!("Hit {} trees!", ntrees);
}
