// https://www.codingame.com/ide/puzzle/don't-panic-episode-1

use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq)]
enum CellType {
    Empty,
    Elevator,
    Clone, // A robot
}

struct Cell {
    pub t: CellType,
    pub position: Point,
}

struct Game {
    pub width: i32,
    pub heigh: i32,
    pub grid: Vec<Cell>,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Self {
        let mut game = Game {
            width: w,
            heigh: h,
            grid: Vec::with_capacity((w * h) as usize),
        };

        for y in 0..h {
            for x in 0..w {
                game.grid.push(Cell {
                    t: CellType::Empty,
                    position: Point { x, y },
                })
            }
        }

        game
    }

    pub fn get_cell(&self, point: &Point) -> &Cell {
        let cell = &self.grid[(point.y * self.width + point.x) as usize];
        assert!(
            cell.position.x == point.x,
            "cell.x({}) != point.x({})",
            cell.position.x,
            point.x
        );
        assert!(
            cell.position.y == point.y,
            "cell.y({}) != point.y({})",
            cell.position.y,
            point.y
        );
        cell
    }

    pub fn get_mut_cell(&mut self, point: &Point) -> &mut Cell {
        let cell = &mut self.grid[(point.y * self.width + point.x) as usize];
        assert!(
            cell.position.x == point.x,
            "cell.x({}) != point.x({})",
            cell.position.x,
            point.x
        );
        assert!(
            cell.position.y == point.y,
            "cell.y({}) != point.y({})",
            cell.position.y,
            point.y
        );
        cell
    }

    pub fn set_cell_type(&mut self, pos: &Point, t: CellType) {
        self.get_mut_cell(pos).t = t;
    }

    pub fn get_next_elevator_position(&self, y: i32) -> Option<Point> {
        for x in 0..self.width {
            let cell = self.get_cell(&Point { x, y });
            if cell.t == CellType::Elevator {
                return Some(cell.position.clone());
            }
        }
        None
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let nb_floors = parse_input!(inputs[0], i32); // number of floors
    let width = parse_input!(inputs[1], i32); // width of the area
    let mut game = Game::new(width, nb_floors);
    let nb_rounds = parse_input!(inputs[2], i32); // maximum number of rounds
    let exit_floor = parse_input!(inputs[3], i32); // floor on which the exit is found
    let exit_pos = parse_input!(inputs[4], i32); // position of the exit on its floor
    let nb_total_clones = parse_input!(inputs[5], i32); // number of generated clones
    let nb_additional_elevators = parse_input!(inputs[6], i32); // ignore (always zero)
    let nb_elevators = parse_input!(inputs[7], i32); // number of elevators
    for _ in 0..nb_elevators as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let elevator_floor = parse_input!(inputs[0], i32); // floor on which this elevator is found
        let elevator_pos = parse_input!(inputs[1], i32); // position of the elevator on its floor
        game.set_cell_type(
            &Point::new(elevator_pos, elevator_floor),
            CellType::Elevator,
        );
    }
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let clone_floor = parse_input!(inputs[0], i32); // floor of the leading clone
        let clone_pos = parse_input!(inputs[1], i32); // position of the leading clone on its floor
        let direction = inputs[2].trim().to_string(); // direction of the leading clone: LEFT or RIGHT

        if direction.as_str() == "NONE" {
            println!("WAIT");
            continue;
        }

        // if spawn_pos_x != clone_pos || spawn_pos_y != clone_floor {
        if clone_floor == exit_floor {
            match direction.as_str() {
                "LEFT" => {
                    if clone_pos < exit_pos {
                        println!("BLOCK");
                        continue;
                    }
                }
                "RIGHT" => {
                    if clone_pos > exit_pos {
                        println!("BLOCK");
                        continue;
                    }
                }
                _ => panic!("unknown direction: {}", direction),
            }
        } else if let Some(next_elevator_position) = game.get_next_elevator_position(clone_floor) {
            match direction.as_str() {
                "LEFT" => {
                    if clone_pos < next_elevator_position.x {
                        println!("BLOCK");
                        continue;
                    }
                }
                "RIGHT" => {
                    if clone_pos > next_elevator_position.x {
                        println!("BLOCK");
                        continue;
                    }
                }
                _ => panic!("unknown direction: {}", direction),
            }
        }
        // }

        println!("WAIT"); // action: WAIT or BLOCK
                          // Write an action using println!("message...");
                          // To debug: eprintln!("Debug message...");
    }
}
