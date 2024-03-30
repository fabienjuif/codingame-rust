// https://www.codingame.com/training/hard/the-labyrinth

use std::io;

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

    pub fn manhattan_distance(&self, pos: &Point) -> i32 {
        let cost = 1;
        let dx = (self.x - pos.x).abs();
        let dy = (self.y - pos.y).abs();
        cost * (dx + dy)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Start,
    CommandRoom,
    Unknown,
}

impl CellType {
    fn as_char(&self) -> char {
        match self {
            CellType::Empty => '.',
            CellType::Wall => '#',
            CellType::Start => 'T',
            CellType::CommandRoom => 'C',
            CellType::Unknown => '?',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '.' => CellType::Empty,
            '#' => CellType::Wall,
            'T' => CellType::Start,
            'C' => CellType::CommandRoom,
            _ => CellType::Unknown,
        }
    }
}

#[derive(Copy, Clone)]
struct Cell {
    pos: Point,
    cell_type: CellType,
}

impl Cell {
    pub fn new(pos: Point) -> Self {
        Self {
            pos,
            cell_type: CellType::Empty,
        }
    }

    pub fn reset(&mut self) {
        self.cell_type = CellType::Empty;
    }

    pub fn is_visitable(&self) -> bool {
        self.cell_type != CellType::Wall
    }
}

#[derive(Copy, Clone, Debug)]
struct AStarPoint {
    point: Point,
    /// the movement cost to move from the starting point to a given square on the grid, following the path generated to get there.
    g: i32,
    /// the estimated movement cost to move from that given square on the grid to the final destination
    h: i32,
    /// total cost (g + h)
    f: i32,
    parent: Option<Point>,
}

impl PartialEq for AStarPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

#[derive(Clone)]
struct Grid {
    height: i32,
    width: i32,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(w: i32, h: i32) -> Self {
        let mut cells = Vec::with_capacity((w * h) as usize);
        for y in 0..h {
            for x in 0..w {
                cells.push(Cell::new(Point::new(x, y)))
            }
        }
        Self {
            height: h,
            width: w,
            cells: cells,
        }
    }

    pub fn set_cell_type(&mut self, pos: &Point, cell_type: CellType) {
        let Some(cell) = self.get_mut_cell(pos) else {
            return;
        };
        cell.cell_type = cell_type;
    }

    pub fn get_cell_index(&self, pos: &Point) -> usize {
        (self.width * pos.y + pos.x) as usize
    }

    pub fn get_mut_cell(&mut self, pos: &Point) -> Option<&mut Cell> {
        let index = self.get_cell_index(pos);
        self.cells.get_mut(index)
    }

    pub fn get_cell(&self, pos: &Point) -> Option<&Cell> {
        let index = self.get_cell_index(pos);
        self.cells.get(index)
    }

    pub fn get_cell_cost(&self, pos: &Point) -> i32 {
        if let Some(cell) = self.get_cell(pos) {
            return match cell.cell_type {
                // if we can avoid clicking while we are roaming this is better
                CellType::CommandRoom => 10,
                // a wall, should not happens but just to be sure we put a super high cost
                CellType::Wall => i32::MAX,
                _ => 1,
            };
        }
        i32::MAX
    }

    pub fn reset(&mut self) {
        self.cells.clear();
    }

    /// get the 4 directions neighbors, this function can return less than 4 points if cells are not visitables or do no exists
    pub fn get_neighbors_points(&self, pos: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let points = [
            &Point::new(pos.x - 1, pos.y),
            &Point::new(pos.x + 1, pos.y),
            &Point::new(pos.x, pos.y - 1),
            &Point::new(pos.x, pos.y + 1),
        ];
        for point in points {
            if let Some(cell) = self.get_cell(point) {
                if cell.is_visitable() {
                    neighbors.push(cell.pos);
                }
            }
        }
        neighbors
    }

    /// returns the points to follow using a* in a grid where we can go over 4 directions
    ///
    /// documentation: http://theory.stanford.edu/~amitp/GameProgramming/ImplementationNotes.html
    /// documentation 2: https://www.geeksforgeeks.org/a-search-algorithm/
    pub fn astar(&self, start: &Point, target: &Point, debug: bool) -> Vec<Point> {
        let mut open = Vec::<AStarPoint>::new();
        let mut closed = Vec::<AStarPoint>::new();
        open.push(AStarPoint {
            point: *start,
            parent: None,
            g: 0,
            h: 0,
            f: 0,
        });

        while open.len() > 0 {
            open.sort_by(|a, b| b.f.cmp(&a.f));
            let q = open.pop().unwrap();
            closed.push(q);

            if debug {
                eprintln!(
                    "open: {:?}",
                    open.iter().map(|c| c.point).collect::<Vec<_>>()
                );
            }

            for neighbor_point in self.get_neighbors_points(&q.point) {
                let mut astar_neighbor = AStarPoint {
                    g: 0,
                    h: 0,
                    f: 0,
                    parent: Some(q.point.clone()),
                    point: neighbor_point,
                };

                if &astar_neighbor.point == target {
                    let mut path = Vec::new();
                    let mut astar_point = astar_neighbor;
                    path.push(*target);
                    while let Some(parent_point) = astar_point.parent {
                        let position = closed.iter().position(|p| p.point == parent_point).expect(
                            format!(
                                "WHAT ASTAR POINT NOT FOUND IN CLOSED QUEUE? {:?} | {:?}",
                                parent_point, closed,
                            )
                            .as_str(),
                        );
                        astar_point = closed.get(position).unwrap().clone();
                        path.push(astar_point.point);
                    }
                    if let Some(last) = path.pop() {
                        if &last != start {
                            path.push(last);
                        }
                    }
                    path.reverse();
                    eprintln!("a* path: {:?}", path);
                    return path;
                }

                astar_neighbor.g = q.g + self.get_cell_cost(&astar_neighbor.point);
                astar_neighbor.h = target.manhattan_distance(&astar_neighbor.point);
                astar_neighbor.f = astar_neighbor.g + astar_neighbor.h;

                if let Some(position) = open.iter().position(|p| p.point == neighbor_point) {
                    if open.get(position).unwrap().f < astar_neighbor.f {
                        continue;
                    } else {
                        open.swap_remove(position);
                    }
                }

                if let Some(position) = closed.iter().position(|p| p.point == neighbor_point) {
                    if closed.get(position).unwrap().f > astar_neighbor.f {
                        closed.swap_remove(position);
                        open.push(astar_neighbor);
                    }
                } else {
                    open.push(astar_neighbor);
                }
            }
        }

        return Vec::new();
    }

    fn set_unknown_as_wall(&mut self) {
        for cell in &mut self.cells {
            if cell.cell_type == CellType::Unknown {
                cell.cell_type = CellType::Wall;
            }
        }
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("");
        for y in 0..self.height {
            for x in 0..self.width {
                str.push(
                    self.get_cell(&Point::new(x, y))
                        .map(|c| c.cell_type.as_char())
                        .unwrap_or('?'),
                );
            }
            str.push('\n');
        }
        f.write_str(str.as_str())
    }
}

struct Game {
    grid: Grid,
    alarm: i32,
    round: i32,
    player_pos: Point,
    command_pos: Option<Point>,
    start_pos: Point,
    hit_command: bool,
    roam_to: Option<Point>,
}

impl Game {
    pub fn new(w: i32, h: i32, alarm: i32) -> Self {
        Self {
            alarm,
            grid: Grid::new(w, h),
            round: 0,
            player_pos: Point::new(0, 0),
            command_pos: None,
            start_pos: Point::new(0, 0),
            hit_command: false,
            roam_to: None,
        }
    }

    pub fn set_player_pos(&mut self, player_pos: Point) {
        self.round += 1;
        self.player_pos = player_pos;
        if let Some(command_pos) = self.command_pos {
            if self.player_pos == command_pos {
                self.hit_command = true;
            }
        }
    }

    pub fn decode_row(&mut self, y: i32, row: String) {
        for (x, c) in row.chars().enumerate() {
            let cell_type = CellType::from_char(c);
            let pos = &Point::new(x as i32, y);
            self.grid.set_cell_type(pos, cell_type.clone());
            match cell_type {
                CellType::CommandRoom => self.command_pos = Some(*pos),
                CellType::Start => self.start_pos = *pos,
                _ => {}
            }
        }
    }

    pub fn get_next_target_point(&mut self) -> Option<Point> {
        // hit command, this is retrieve mode, just find the fatest path using a*
        if self.hit_command {
            eprintln!("--MODE: RETURN TO STARTING POSITION: {:?}", self.start_pos);
            // set all unknown cells as walls to avoid discovering new path and wait time
            self.grid.set_unknown_as_wall();

            return self
                .grid
                .astar(&self.player_pos, &self.start_pos, false)
                .get(0)
                .copied()
                .or_else(|| Some(self.start_pos));
        }

        // roaming mode
        // we are looking for every ? - if there is at least one targetable
        // from the closest to the player to the furthest
        if let Some(roam_to) = self.roam_to {
            if let Some(point) = self.grid.astar(&self.player_pos, &roam_to, false).get(0) {
                if point != &self.player_pos
                // makes sure the target cell (roam_to) is not discovered while we moved to it
                    && self.grid.get_cell(&roam_to).unwrap().cell_type == CellType::Unknown
                {
                    eprintln!(">> STRAIGHT TO: {:?}", roam_to);
                    return Some(point.clone());
                }
            }
        }
        let mut unknown_cells = self
            .grid
            .cells
            .iter()
            .filter(|cell| cell.cell_type == CellType::Unknown)
            .collect::<Vec<_>>();
        unknown_cells.sort_by(|&a, &b| {
            b.pos
                .manhattan_distance(&self.player_pos)
                .cmp(&a.pos.manhattan_distance(&self.player_pos))
        });
        let mut tries = 0;
        while let Some(cell) = unknown_cells.pop() {
            tries += 1;
            if tries > 4 {
                break;
            }
            eprintln!("closest UNKNOWN cell to roam: {:?}", cell.pos);
            if let Some(point) = self.grid.astar(&self.player_pos, &cell.pos, false).get(0) {
                eprintln!(">> NEXT TARGET: {:?}", point);
                self.roam_to = Some(cell.pos);
                return Some(point.clone());
            }
        }

        // nothing more to roam, we should have found the command
        // just press it
        // we have found the command pos, just use a* to go there
        if let Some(command_pos) = &self.command_pos {
            eprintln!("--MODE: GO TO COMMAND STATION: {:?}", command_pos);
            return self
                .grid
                .astar(&self.player_pos, command_pos, false)
                .get(0)
                .copied();
        }

        None
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    // init game
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i32); // number of rows.
    let c = parse_input!(inputs[1], i32); // number of columns.
    let a = parse_input!(inputs[2], i32); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.
    let mut game = Game::new(c, r, a);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let kr = parse_input!(inputs[0], i32); // row where Rick is located.
        let kc = parse_input!(inputs[1], i32); // column where Rick is located.
        game.set_player_pos(Point::new(kc, kr));

        for y in 0..r as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim().to_string(); // C of the characters in '#.TC?' (i.e. one line of the ASCII maze).
            game.decode_row(y as i32, row);
        }

        eprintln!("{:?}", game.grid);

        if let Some(next_point) = game.get_next_target_point() {
            eprintln!("next_point: {:?}", next_point);
            if next_point.x == game.player_pos.x {
                if next_point.y <= game.player_pos.y {
                    println!("UP");
                } else {
                    println!("DOWN");
                }
            } else {
                if next_point.x <= game.player_pos.x {
                    println!("LEFT");
                } else {
                    println!("RIGHT");
                }
            }
        } else {
            eprintln!("WHAT I DUNNO WHAT TO DO");
            println!("RIGHT"); // Rick's next move (UP DOWN LEFT or RIGHT).
        }
    }
}
