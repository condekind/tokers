use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // TODO: add a check to test if world.gen.rs already exists
    //       bonus: also check if we need to update it
    let map_name = "World00";
    let map_file_path = format!("./data/{map_name}.map");

    // Open file
    let path = Path::new(&map_file_path);
    let mut file = File::open(path).expect("Unable to open file");

    // Read from file to String
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Unable to read file");

    // Count lines (num of rows), count characters from 1st row (num of cols)
    let lines: Vec<&str> = s.split('\n').collect();
    let rows = lines.len();
    let cols = lines.first().map(|line| line.chars().count()).unwrap_or(0);

    // Create file that will contain generated code with the world information
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("world.gen.rs");
    let mut f = File::create(dest_path).unwrap();

    // Write generated code to the file created above
    f.write_all(
        format!(
            "\
            const NUM_GRID_ROWS: usize = {rows};\n\
            const NUM_GRID_COLS: usize = {cols};\n\
            pub type Grid2D{map_name} = [[Tile; {cols}]; {rows}];\
        "
        )
        .as_bytes(),
    )
    .unwrap();
}
