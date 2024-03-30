// https://www.codingame.com/ide/puzzle/code-vs-zombies

use std::{collections::HashMap, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get_squared_distance(&self, pos: &Point) -> i32 {
        (pos.y - self.y).pow(2) + (pos.x - self.x).pow(2)
    }

    pub fn get_distance(&self, pos: &Point) -> f32 {
        (self.get_squared_distance(pos) as f32).sqrt()
    }
}

#[derive(Debug)]
struct Distance {
    from: Point,
    to: Point,
    distance: f32,
}

impl Distance {
    pub fn new(from: Point, to: Point) -> Self {
        Distance {
            from,
            to,
            distance: from.get_distance(&to),
        }
    }

    pub fn new_with_distance(from: Point, to: Point, distance: f32) -> Self {
        Distance { from, to, distance }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum CellType {
    Empty,
    Human,
    Zombie,
}

#[derive(Copy, Clone)]
struct Cell {
    pos: Point,
    cell_type: CellType,
    zombies_count: usize,
}

impl Cell {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            cell_type: CellType::Empty,
            zombies_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cell_type = CellType::Empty;
        self.zombies_count = 0;
    }
}

#[derive(Clone)]
struct Grid {
    height: i32,
    width: i32,
    cells: HashMap<Point, Cell>,
}

impl Grid {
    pub fn new(w: i32, h: i32) -> Self {
        let cells = HashMap::with_capacity((w * h) as usize);
        Self {
            height: h,
            width: w,
            cells: cells,
        }
    }

    pub fn set_cell_type(&mut self, pos: &Point, cell_type: CellType) {
        let cell = self.get_mut_cell(pos);
        cell.cell_type = cell_type;
        if cell.cell_type == CellType::Zombie {
            cell.zombies_count += 1;
        }
    }

    pub fn get_mut_cell(&mut self, pos: &Point) -> &mut Cell {
        if self.cells.contains_key(pos) {
            return self.cells.get_mut(pos).unwrap();
        }

        self.cells.insert(*pos, Cell::new(*pos));
        self.cells.get_mut(pos).unwrap()
    }

    pub fn reset(&mut self) {
        self.cells.clear();
    }
}

#[derive(Copy, Clone, Debug)]
struct Human {
    id: i32,
    pos: Point,

    turns_to_zombie: f32,
    turns_to_player: f32,
}

impl Human {
    pub fn new(id: i32, pos: Point) -> Self {
        Self {
            id,
            pos,
            turns_to_zombie: 0.,
            turns_to_player: 0.,
        }
    }
}

#[derive(Copy, Clone)]
struct Zombie {
    id: i32,
    pos: Point,
    next_pos: Point,
}

impl Zombie {
    pub fn new(id: i32, pos: Point, next_pos: Point) -> Self {
        Self { id, pos, next_pos }
    }
}

#[derive(Clone)]
struct Game {
    player_pos: Point,

    player_shoot_distance: i32,
    player_velocity: i32,
    zombie_velocity: i32,

    prev_humans: HashMap<i32, Human>,
    prev_zombies: HashMap<i32, Zombie>,

    humans: HashMap<i32, Human>,
    zombies: HashMap<i32, Zombie>,

    grid: Grid,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            player_shoot_distance: 2000,
            player_velocity: 1000,
            zombie_velocity: 400,
            player_pos: Point::new(-1, -1),
            humans: HashMap::new(),
            grid: Grid::new(w, h),
            zombies: HashMap::new(),
            prev_humans: HashMap::new(),
            prev_zombies: HashMap::new(),
        }
    }

    /// reset grid for a new loop
    pub fn new_loop(&mut self) {
        self.grid.reset();
        self.prev_humans = self.humans.clone();
        self.prev_zombies = self.zombies.clone();
        self.humans.clear();
        self.zombies.clear();
    }

    /// adds a human, if a human has been deleted, put it in dead_humans vec (useful for debug)
    pub fn add_human(&mut self, human: Human) {
        self.grid.set_cell_type(&human.pos, CellType::Human);
        self.humans.insert(human.id, human);
    }

    /// adds a zombie, if a zombie has been deleted, put it in dead_zombies vec (useful for debug)
    pub fn add_zombie(&mut self, zombie: Zombie) {
        self.grid.set_cell_type(&zombie.pos, CellType::Zombie);
        self.zombies.insert(zombie.id, zombie);
    }

    /// compute for each humans how many turns he could live
    pub fn compute_humans_life_expectancy(&mut self) {
        for (_, human) in self.humans.iter_mut() {
            let closest_zombie_pos =
                Game::get_closest_zombie_next_pos(self.zombies.values().collect(), &human.pos);
            human.turns_to_zombie =
                (closest_zombie_pos.distance as f32) / (self.zombie_velocity as f32);
        }
    }

    /// compute each human distance to player (in turns)
    pub fn compute_player_to_humans_turns(&mut self) {
        for (_, human) in &mut self.humans {
            human.turns_to_player = ((self.player_pos.get_distance(&human.pos) as f32)
                - (self.player_shoot_distance as f32))
                / (self.player_velocity as f32);
        }
    }

    pub fn get_closest_zombie_next_pos(zombies: Vec<&Zombie>, pos: &Point) -> Distance {
        let mut closest_distance = -1.;
        let mut closest_point = Point::new(0, 0);
        for zombie in zombies {
            let distance = zombie.next_pos.get_distance(pos);
            if closest_distance == -1. || closest_distance > distance {
                closest_distance = distance;
                closest_point = zombie.pos;
            }
        }
        Distance::new_with_distance(*pos, closest_point, closest_distance)
    }

    pub fn get_closest_rescuable_human_pos(&self) -> Option<Point> {
        let mut rescuable_humans = self
            .humans
            .values()
            .filter(|&human| human.turns_to_player < human.turns_to_zombie)
            .collect::<Vec<_>>();
        rescuable_humans.sort_by(|a, b| a.turns_to_player.total_cmp(&b.turns_to_player));
        rescuable_humans.get(0).map(|human| human.pos)
    }
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    let mut game = Game::new(16000, 9000);

    // game loop
    loop {
        game.new_loop();

        // player pos
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        game.player_pos.x = x;
        game.player_pos.y = y;

        // humans
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        for _ in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            game.add_human(Human::new(human_id, Point::new(human_x, human_y)));
        }

        // zombies
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        for _ in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);
            game.add_zombie(Zombie::new(
                zombie_id,
                Point::new(zombie_x, zombie_y),
                Point::new(zombie_xnext, zombie_ynext),
            ));
        }

        // some computes
        game.compute_humans_life_expectancy();
        game.compute_player_to_humans_turns();

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        if let Some(pos) = game.get_closest_rescuable_human_pos() {
            println!("{} {}", pos.x, pos.y); // Your destination coordinates
        } else {
            println!("0 0 FUCK ME");
        }
    }
}
