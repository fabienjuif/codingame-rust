// https://www.codingame.com/ide/puzzle/shadows-of-the-knight-episode-1

use std::{cmp, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let mut x = parse_input!(inputs[0], i32);
    let mut y = parse_input!(inputs[1], i32);

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = w;
    let mut max_y = h;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        let bx = x;
        let by = y;

        match bomb_dir.as_str() {
            "U" => {
                max_y = y + 1;
                y -= cmp::max(1, (max_y - min_y) / 2);
            }
            "UR" => {
                // TODO: maybe use pythagore instead (proper distance)
                max_y = y + 1;
                min_x = x;
                y -= cmp::max(1, (max_y - min_y) / 2);
                x += cmp::max(1, (max_x - min_x) / 2);
            }
            "R" => {
                min_x = x;
                x += cmp::max(1, (max_x - min_x) / 2);
            }
            "DR" => {
                min_y = y;
                min_x = x;
                y += cmp::max(1, (max_y - min_y) / 2);
                x += cmp::max(1, (max_x - min_x) / 2);
            }
            "D" => {
                min_y = y;
                y += cmp::max(1, (max_y - min_y) / 2);
            }
            "DL" => {
                min_y = y;
                max_x = x + 1;
                y += cmp::max(1, (max_y - min_y) / 2);
                x -= cmp::max(1, (max_x - min_x) / 2);
            }
            "L" => {
                max_x = x + 1;
                x -= cmp::max(1, (max_x - min_x) / 2);
            }
            "UL" => {
                max_y = y + 1;
                max_x = x + 1;
                y -= cmp::max(1, (max_y - min_y) / 2);
                x -= cmp::max(1, (max_x - min_x) / 2);
            }
            _ => {
                panic!("unknown dir: {}", bomb_dir);
            }
        }

        eprintln!(
            "b:{}/x:{};y:{}/min_x:{};min_y:{}/max_x:{};max_y:{}",
            bomb_dir, bx, by, min_x, min_y, max_x, max_y
        );
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // the location of the next window Batman should jump to.
        println!("{} {}", x, y);
    }
}
