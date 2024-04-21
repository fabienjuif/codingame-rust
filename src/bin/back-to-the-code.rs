// https://www.codingame.com/multiplayer/bot-programming/back-to-the-code
// 2527
// 1411
// 531
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::io;

const HEIGHT: i32 = 20;
const WIDTH: i32 = 35;
const MAX_DISTANCE: f64 = 1625.;
const STARTING_SQUARE_WIDTH: i32 = 18;

// closest possible to enemies
// to the cell I can get without them getting it before
// flood fill?
// OR
// try to make rectangle (from the bigger I can to the smaller)
// once the rectangle is found (I can go to each cell before enemies), I draw it
// starting by the closest point to enemies
// 1. start with squares
// 2. then find the max height then max width to do a rectangle
//

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Default, Debug)]
struct Player {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub back_in_time_left: i32,
}

impl Player {
    pub fn init(&mut self, id: i32, input_line: String) -> &mut Self {
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        self.id = id;
        self.x = parse_input!(inputs[0], i32);
        self.y = parse_input!(inputs[1], i32);
        self.back_in_time_left = parse_input!(inputs[2], i32);
        self
    }

    pub fn is_at(&self, cell: &Cell) -> bool {
        self.x == cell.x && self.y == cell.y
    }
}

#[derive(Clone)]
struct Cell {
    pub x: i32,
    pub y: i32,
    pub player_id: i32,
    pub debug_char: char,
    /**
     * contains a map player_id -> distance
     * distance is -1 if the cell is owned by a player
     */
    pub distances: HashMap<i32, f64>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.player_id == other.player_id
    }
}

impl Eq for Cell {}

impl Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.player_id.hash(state);
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            player_id: -1,
            debug_char: char::default(),
            distances: HashMap::new(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.player_id)
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cell")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("player_id", &self.player_id)
            .finish()
    }
}

impl Cell {
    /** distance is not SQRT at the end */
    pub fn distance_point(&self, x: i32, y: i32) -> f64 {
        f64::from(self.x - x).powi(2) + f64::from(self.y - y).powi(2)
    }

    pub fn is_at(&self, other: &Cell) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Grid {
    pub cells: Vec<Cell>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last_y: i32 = 0;
        for cell in self.cells.iter() {
            if cell.y != last_y {
                write!(f, "\n").unwrap();
            }
            last_y = cell.y;
            if cell.debug_char != char::default() {
                write!(f, "{}", cell.debug_char).unwrap();
            } else if cell.player_id < 0 {
                write!(f, ".").unwrap();
            } else {
                write!(f, "{}", cell.player_id).unwrap();
            };
        }
        write!(f, "\n")
    }
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Grid {
            cells: Vec::<Cell>::with_capacity((HEIGHT * WIDTH) as usize),
        };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                grid.cells.push(Cell {
                    x: x,
                    y: y,
                    player_id: -1,
                    ..Default::default()
                })
            }
        }
        grid
    }
}

impl Grid {
    pub fn get_cell_index(x: i32, y: i32) -> usize {
        (y * WIDTH + x) as usize
    }
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        if x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT {
            return None;
        }
        self.cells.get(Grid::get_cell_index(x, y))
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        if x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT {
            return None;
        }
        self.cells.get_mut(Grid::get_cell_index(x, y))
    }

    pub fn compute_all_distances_player(&mut self, player_id: i32, x: i32, y: i32) {
        for cell in self.cells.iter_mut() {
            if cell.player_id >= 0 {
                cell.distances.insert(player_id, -1.);
                continue;
            }
            cell.distances.insert(player_id, cell.distance_point(x, y));
        }
    }

    pub fn debug_distances(&self, player_id: i32) -> String {
        let mut str = String::from("");
        let mut last_y: i32 = 0;
        for cell in self.cells.iter() {
            if cell.y != last_y {
                str = format!("{}\n", str);
                last_y = cell.y;
            }
            str = format!(
                "{}{}|",
                str,
                cell.distances
                    .get(&player_id)
                    .map_or(String::from("."), |v| if v < &0. {
                        String::from("x")
                    } else {
                        format!("{:.0}", ((v / MAX_DISTANCE) * 10.).round())
                    })
            );
        }
        format!("{}\n", str)
    }

    /// Return the fitting perimeters (all cells in the perimeter) if the square of width w can be hold in position and nothing is inside except given player own cells.
    /// `None` if it can not.
    ///
    /// - The square is drawn right down to the given position (x,y is the top/left corner)
    /// - All given player cells are considered free cells, meaning this does not check if we already have started the drawing of the square.
    ///     - BUT if none of the cells is free it returns `None`
    pub fn get_fitting_perimeter(
        &self,
        w: i32,
        h: i32,
        x: i32,
        y: i32,
        player_id: i32,
    ) -> Option<Vec<Cell>> {
        let mut perimeter = HashSet::<Cell>::new(); // TODO: not sure we need hashset because we should not pick twice the same cell, but was lazy
        for relative_x in 0..w {
            for relative_y in 0..h {
                let Some(cell) = self.get_cell(relative_x + x, relative_y + y) else {
                    return None;
                };

                // cell is owned by a player and this is not us -> does not fit
                if cell.player_id >= 0 && cell.player_id != player_id {
                    return None;
                }

                // if this cell is part of perimeter we add it
                if (relative_x == 0 || relative_x == w - 1)
                    || (relative_y == 0 || relative_y == h - 1)
                {
                    perimeter.insert(cell.clone());
                }
            }
        }

        for cell in perimeter.clone().into_iter() {
            if cell.player_id <= -1 {
                return Some(perimeter.into_iter().collect());
            }
        }

        None
    }

    pub fn from_sub_grid(cells: Vec<Cell>) -> Self {
        let mut grid = Grid::default();
        for mut cell in cells {
            let x = cell.x;
            let y = cell.y;
            if cell.player_id < 0 {
                cell.debug_char = 'x';
            }
            grid.cells[Grid::get_cell_index(x, y)] = cell.clone()
        }
        grid
    }

    pub fn get_min_xy(cells: Vec<Cell>) -> Option<[i32; 2]> {
        let mut x = -1;
        let mut y = -1;
        for cell in &cells {
            if x == -1 || cell.x < x {
                x = cell.x
            }
            if y == -1 || cell.y < y {
                y = cell.y
            }
        }
        if x == -1 || y == -1 {
            return None;
        }
        Some([x, y])
    }

    pub fn get_max_xy(cells: Vec<Cell>) -> Option<[i32; 2]> {
        let mut x = -1;
        let mut y = -1;
        for cell in &cells {
            if x == -1 || cell.x > x {
                x = cell.x
            }
            if y == -1 || cell.y > y {
                y = cell.y
            }
        }
        if x == -1 || y == -1 {
            return None;
        }
        Some([x, y])
    }
}

#[derive(Default)]
struct Game {
    pub round: i32,
    pub players: Vec<Player>,
    pub grid: Grid,
}

impl Game {
    /**
     * set player input, player with id: 0 is us, others are opponents.
     */
    pub fn set_player_inputs(&mut self, index: usize, input_line: String) -> &mut Self {
        if let Some(player) = self.players.get_mut(index) {
            player.init(index as i32, input_line);
            return self;
        }
        self.players.push(Player::default());
        self.set_player_inputs(index, input_line)
    }

    pub fn set_grid_line(&mut self, index: usize, input_line: String) -> &mut Self {
        for (x, value) in input_line.trim().chars().enumerate() {
            let Some(cell) = self.grid.get_cell_mut(x as i32, index as i32) else {
                continue;
            };
            match value {
                '.' => cell.player_id = -1,
                _ => cell.player_id = value.to_string().parse::<i32>().unwrap(),
            }
        }

        self
    }

    pub fn compute_distances(&mut self) -> &mut Self {
        for player in self.players.iter() {
            self.grid
                .compute_all_distances_player(player.id, player.x, player.y);
        }
        self
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let opponent_count = parse_input!(input_line, usize);
    let mut game = Game::default();

    let mut best_perimeter = Option::<Vec<Cell>>::None;
    let mut best_perimeter_x = 0;
    let mut best_perimeter_y = 0;
    let mut best_perimeter_w = 0;
    let mut best_perimeter_h = 0;

    let mut targeted_cell = Option::<Cell>::None;

    // game loop
    loop {
        // parse inputs
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        game.round = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        game.set_player_inputs(0, input_line);
        for i in 0..opponent_count {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            game.set_player_inputs(i + 1, input_line);
        }
        for i in 0..HEIGHT as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            game.set_grid_line(i, input_line);
        }

        // some computations
        game.compute_distances();

        for i in 0..game.players.len() {
            eprintln!("player {} distances:", i);
            eprintln!("{}", game.grid.debug_distances(i as i32));
        }
        let get_best_perimeter_for_w = |w: i32, h: i32| {
            let player = game.players.get(0).unwrap();
            let cells = game
                .grid
                .get_fitting_perimeter(w, h, player.x - w + 1, player.y - h + 1, player.id)
                .or_else(|| {
                    game.grid
                        .get_fitting_perimeter(w, h, player.x, player.y - h + 1, player.id)
                })
                .or_else(|| {
                    game.grid
                        .get_fitting_perimeter(w, h, player.x - w + 1, player.y, player.id)
                })
                .or_else(|| {
                    game.grid
                        .get_fitting_perimeter(w, h, player.x, player.y, player.id)
                });
            if cells.is_none() {
                for x in 0..WIDTH {
                    for y in 0..HEIGHT {
                        if let Some(cells) = game.grid.get_fitting_perimeter(w, h, x, y, player.id)
                        {
                            return Option::Some(cells);
                        }
                    }
                }
            }
            cells
        };

        let get_best_perimeter = || {
            let max_w: i32 = WIDTH - 1;
            let max_h: i32 = HEIGHT - 1;

            for w in (2..max_w).rev() {
                for h in (w..max_h).rev() {
                    let perimeter = get_best_perimeter_for_w(w, h);
                    if perimeter.is_some() {
                        return perimeter;
                    }
                }
            }

            None
        };

        // find the best perimeter we keep for multiple loop
        let mut set_best_perimeter = || {
            if best_perimeter.is_none() {
                if let Some(mut cells) = get_best_perimeter() {
                    cells.sort_by(|a, b| b.y.cmp(&a.y.clone()));
                    cells.sort_by(|a, b| b.x.cmp(&a.x.clone()));
                    let sub_grid = Grid::from_sub_grid(cells.clone());
                    if let Some(min_xy) = Grid::get_min_xy(cells.clone()) {
                        best_perimeter_x = min_xy[0];
                        best_perimeter_y = min_xy[1];
                        if let Some(max_xy) = Grid::get_max_xy(cells.clone()) {
                            best_perimeter_w = max_xy[0] - min_xy[0];
                            best_perimeter_h = max_xy[1] - min_xy[1];
                        }
                    }
                    best_perimeter = Some(cells);

                    eprintln!("{}", sub_grid);
                }
            }
        };

        let mut printed = false;

        set_best_perimeter();
        if let Some(cells) = &best_perimeter {
            let cloned_targeted_cell = targeted_cell.clone();

            // go to the closest cells of the player that is not already drawn
            let mut set_targeted_cell = || {
                let mut cells = cells.clone();
                cells.sort_by(|a, b| {
                    game.grid
                        .get_cell(a.x, a.y)
                        .unwrap()
                        .distances
                        .get(&0)
                        .unwrap()
                        .partial_cmp(
                            game.grid
                                .get_cell(b.x, b.y)
                                .unwrap()
                                .distances
                                .get(&0)
                                .unwrap(),
                        )
                        .unwrap()
                });

                let mut cell = cells[0].clone();
                let mut idx = 1;
                while game.grid.get_cell(cell.x, cell.y).unwrap().player_id >= 0
                    && idx < cells.len()
                {
                    cell = cells[idx].clone();
                    idx += 1;
                }
                targeted_cell = Some(cell);
            };

            if cloned_targeted_cell.is_none() {
                set_targeted_cell();
            } else {
                let cell = cloned_targeted_cell.unwrap();
                eprintln!(
                    "p.x={};p.y={} - t.x={};t.y={}",
                    game.players[0].x, game.players[0].y, cell.x, cell.y
                );
                if game.players[0].is_at(&cell) {
                    set_targeted_cell();
                }
            }

            if let Some(cell) = &targeted_cell {
                match game.grid.get_cell(cell.x, cell.y) {
                    None => {
                        best_perimeter = None;
                        targeted_cell = None;
                    }
                    Some(cell) => {
                        printed = true;
                        println!("{} {}", cell.x, cell.y);
                        // eprintln!("{}", game.grid);
                        eprintln!(
                            "w={};h={};x={};y={}",
                            best_perimeter_w, best_perimeter_h, best_perimeter_x, best_perimeter_y
                        );
                        if cell.player_id > 0
                            || game
                                .grid
                                .get_fitting_perimeter(
                                    best_perimeter_w,
                                    best_perimeter_h,
                                    best_perimeter_x,
                                    best_perimeter_y,
                                    0,
                                )
                                .is_none()
                        {
                            best_perimeter = None;
                            targeted_cell = None;
                        }
                    }
                }
            }
        }

        if !printed {
            println!("0 0 - FUCK");
        }
    }
}
