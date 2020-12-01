use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {

    let args = Cli::from_args();

    let input: Vec<_> = std::fs::read_to_string(args.input)
        .expect("Failed to read input file.")
        .lines()
        .map(|v| v.parse::<i32>().expect("Failed to parse number."))
        .collect();

    // let's start dumb...
    // x + y = target
    let target = 2020i32;
    let mut v = [-1i32, -1i32];
    for (ii, x) in input.iter().enumerate() {
        let y_target = target - x;
        let input2 = &input[ii..];
        for y in input2 {
            // println!("Checking y = {} == {}", *y, y_target);
            if *y == y_target {
                println!("Found {} + {} = {}", x, y, target);
                v = [*x, *y];
                break;
            }
        }
        if v[0] == *x { break; }
    }

    println!("Product = {}", v[0] * v[1]);

}
