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

    let width = input.lines().next().unwrap().len();
    let movements = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1u64;

    for movement in movements.iter() {
        let mut pos = (0usize, 0usize);
        let ntrees = input
            .lines()
            .step_by(movement.1 as usize)
            .fold(0, |count, line| {
                let mut new_count = count;
                if line.chars().nth(pos.0).unwrap() == '#' {
                    new_count += 1;
                }
                pos.0 = (pos.0 + movement.0 as usize) % width;
                pos.1 += movement.1 as usize;
                new_count
            });
    
        println!("Right {}, down {} -> Hit {} trees!", movement.0, movement.1, ntrees);
        product *= ntrees;
    }

    println!("product = {}", product);
}
