use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod world;
use world::*;

// Define the player character
struct Player {
    pos_x: u8,
    pos_y: u8,
    display_char: &'static str,
}

// Function to load player state from file
fn load_player() -> Player {
    if Path::new("./data/player.sav").exists() {
        let mut file = File::open("./data/player.sav").expect("Unable to open file");
        let mut s = String::new();
        file.read_to_string(&mut s).expect("Unable to read file");
        let coords: Vec<&str> = s.trim().split(' ').collect();
        Player {
            pos_x: coords[0].parse().unwrap(),
            pos_y: coords[1].parse().unwrap(),
            display_char: "@",
        }
    } else {
        Player {
            pos_x: 1,
            pos_y: 1,
            display_char: "@",
        }
    }
}

// Function to save player state to file
fn save_player(player: &Player) {
    let mut file = File::create("./data/player.sav").expect("Unable to create file");
    writeln!(file, "{} {}", player.pos_x, player.pos_y).expect("Unable to write file");
}

// Draw the player
fn draw_player(stdout: &mut std::io::Stdout, player: &Player) {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(player.pos_x as u16 + 1, player.pos_y as u16 + 1),
        player.display_char
    )
    .unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Currently, World::new() parses info from a world file at runtime to fill
    // a grid with appropriate Tiles. The grid, however, has its dimensions
    // known at compile time, since both NUM_GRID_COLS and NUM_GRID_ROWS are
    // obtained in build.rs, which also parses the same world/map file.
    // We could do everything in comptime (build.rs), but just having the
    // dimensions known (for stack allocation) will do for now. What was done
    // is probably a bad idea already - codegen is very cool but also cursed.
    let world = World::new();
    let mut player = load_player();

    for c in stdin.keys() {
        write!(stdout, "{}", clear::All).unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('w') => {
                if world.grid[(player.pos_y - 1) as usize][player.pos_x as usize].is_walkable() {
                    player.pos_y -= 1
                }
            }
            Key::Char('a') => {
                if world.grid[player.pos_y as usize][(player.pos_x - 1) as usize].is_walkable() {
                    player.pos_x -= 1
                }
            }
            Key::Char('s') => {
                if world.grid[(player.pos_y + 1) as usize][player.pos_x as usize].is_walkable() {
                    player.pos_y += 1
                }
            }
            Key::Char('d') => {
                if world.grid[player.pos_y as usize][(player.pos_x + 1) as usize].is_walkable() {
                    player.pos_x += 1
                }
            }
            _ => (),
        }

        // Draws the world grid according to the kind of tile at each position
        draw_world(&mut stdout, &(world.grid));

        // Draws the player character at their position
        draw_player(&mut stdout, &player);

        // Move cursor to (0,0)
        write!(stdout, "\x1b[0;0H").unwrap();

        stdout.flush().unwrap();
    }

    save_player(&player);
}
