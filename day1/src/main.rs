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

    println!("Part 1 ---");
    let mut v = [-1i32, -1i32];
    for (ii, x) in input.iter().enumerate() {
        let y_target = target - x;
        let input2 = &input[ii..];
        for y in input2 {
            if *y == y_target {
                println!("{} + {} = {}", x, y, target);
                v = [*x, *y];
                break;
            }
        }
        if v[0] == *x {
            break;
        }
    }

    println!("Product = {}", v[0] * v[1]);
    println!("----------\n");

    println!("Part 2 ---");
    let mut v = [-1i32, -1i32, -1i32];
    for (ii, x) in input.iter().enumerate() {
        let remaining = target - x;
        for (jj, y) in input[ii..].iter().enumerate() {
            if *y < remaining {
                let remaining = remaining - y;
                for z in input[ii + jj..].iter() {
                    if *z == remaining {
                        println!("{} + {} + {} = {}", x, y, z, target);
                        v = [*x, *y, *z];
                        break;
                    }
                }
            }
            if v[1] == *y {
                break;
            }
        }
        if v[0] == *x {
            break;
        }
    }
    println!("Product = {}", v[0] * v[1] * v[2]);
    println!("----------\n");
}
