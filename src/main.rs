use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::clear;

// Define the Kind enum
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Wall,
    Grnd
}

// Define the Tile struct
#[derive(Clone, Copy, PartialEq)]
struct Tile {
    kind: Kind,
    walk: bool
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}]", self.kind)
    }
}

// Define the player character
struct Player {
    pos_x: u8,
    pos_y: u8,
    display_char: &'static str,
}

// Function to load world from file
fn load_world(world: &mut [[Tile; 23]; 17]) {
    let path = Path::new("./data/world.dat");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let lines: Vec<&str> = s.split("\n").collect();
    for (i, line) in lines.iter().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            world[i][j] = match cell {
                '#' => Tile{kind: Kind::Wall, walk: false,},
                '.' => Tile{kind: Kind::Grnd, walk: true ,},
                _ => panic!("Invalid cell character"),
            };
        }
    }
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

// Draw the game world
fn draw_world(stdout: &mut std::io::Stdout, world: &[[Tile; 23]; 17]) {
    for i in 0..17 {
        for j in 0..23 {
            let symbol = match world[i][j].kind {
                Kind::Wall => '#',
                Kind::Grnd => '.',
            };
            write!(stdout, "{}{}", termion::cursor::Goto(j as u16 + 1, i as u16 + 1), symbol).unwrap();
        }
    }
}

// Draw the player
fn draw_player(stdout: &mut std::io::Stdout, player: &Player) {
    write!(stdout, "{}{}", termion::cursor::Goto(player.pos_x as u16 + 1, player.pos_y as u16 + 1), player.display_char).unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut world: [[Tile; 23]; 17] = [[Tile {kind: Kind::Grnd, walk: true}; 23]; 17];
    let mut player = load_player();
    load_world(&mut world);

    for c in stdin.keys() {
        write!(stdout, "{}", clear::All).unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('w') => if world[(player.pos_y - 1) as usize][player.pos_x as usize].kind != Kind::Wall { player.pos_y -= 1 },
            Key::Char('a') => if world[player.pos_y as usize][(player.pos_x - 1) as usize].kind != Kind::Wall { player.pos_x -= 1 },
            Key::Char('s') => if world[(player.pos_y + 1) as usize][player.pos_x as usize].kind != Kind::Wall { player.pos_y += 1 },
            Key::Char('d') => if world[player.pos_y as usize][(player.pos_x + 1) as usize].kind != Kind::Wall { player.pos_x += 1 },
            _ => (),
        }
        draw_world(&mut stdout, &world);
        draw_player(&mut stdout, &player);
        write!(stdout, "\x1b[0;0H").unwrap();
        stdout.flush().unwrap();
    }

    save_player(&player);
}

