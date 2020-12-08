use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

enum TerrainType {
    Empty,
    Tree
}

struct Map {
    raw: String,
    // terrain: Vec<TerrainType>,
    height: usize,
    width: usize,
    pos: (usize, usize)
}

impl Map {

    fn new(fname: std::path::PathBuf) -> Map {
        let raw = String::from(std::fs::read_to_string(fname).expect("Failed to read input file."));
        let height = raw.lines().count();
        let width = raw.lines().next().unwrap().len();
        let pos = (0usize, 0usize);

        Map {raw, height, width, pos}
    }
}

fn main() {
    let args = Cli::from_args();
    let map = Map::new(args.input);

    println!("map height = {}, width = {} (periodic)", map.height, map.width);

    // let movement = (3i32, 1i32);

}
