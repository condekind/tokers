use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// --------------------------------- Tiles --------------------------------- //

// Define the Kind enum
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum TileKind {
    #[default]
    Wall,
    Grnd,
}

// Define the Tile struct
#[derive(Clone, Copy, PartialEq, Default)]
pub struct Tile {
    kind: TileKind,
    walk: bool,
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        self.walk
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}]", self.kind)
    }
}

// --------------------------------- World --------------------------------- //

include!(concat!(env!("OUT_DIR"), "/world.gen.rs"));

pub struct World {
    pub grid: Grid2DWorld00,
}

impl World {
    const NUM_ROWS: usize = NUM_GRID_ROWS;
    const NUM_COLS: usize = NUM_GRID_COLS;

    pub fn new() -> Self {
        let mut grid = Grid2DWorld00::default();
        load_world(&mut grid);
        Self { grid }
    }
}

// Function to load world from file
fn load_world(grid: &mut Grid2DWorld00) {
    let path = Path::new("./data/World00.map");
    let display = path.display();
    // Open file
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read to string
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    let lines: Vec<&str> = s.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            grid[i][j] = match cell {
                '#' => Tile {
                    kind: TileKind::Wall,
                    walk: false,
                },
                '.' => Tile {
                    kind: TileKind::Grnd,
                    walk: true,
                },
                _ => panic!("Invalid cell character"),
            };
        }
    }
}

// Draw the game world
pub fn draw_world(stdout: &mut std::io::Stdout, grid: &Grid2DWorld00) {
    for (i, row) in grid.iter().enumerate().take(World::NUM_ROWS) {
        for (j, elem) in row.iter().enumerate().take(World::NUM_COLS) {
            let symbol = match elem.kind {
                TileKind::Wall => '█',
                TileKind::Grnd => '░',
            };
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(j as u16 + 1, i as u16 + 1),
                symbol
            )
            .unwrap();
        }
    }
}
