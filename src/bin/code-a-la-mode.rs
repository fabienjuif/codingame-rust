// https://www.codingame.com/multiplayer/bot-programming/code-a-la-mode

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
    Partner,
    DishWasher,
    Window,
    Blueberries,
    IceCream,
    Strawberries,
    Dough,
    Chopper,
    Oven,
    Unknown,
}

impl CellType {
    fn as_char(&self) -> char {
        match self {
            CellType::Empty => '.',
            CellType::Wall => '#',
            CellType::Partner => 'P',
            CellType::DishWasher => 'D',
            CellType::Window => 'W',
            CellType::Blueberries => 'B',
            CellType::IceCream => 'I',
            CellType::Strawberries => 'S',
            CellType::Dough => 'H',
            CellType::Chopper => 'C',
            CellType::Oven => 'O',
            CellType::Unknown => '?',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '.' | '0' | '1' => CellType::Empty,
            '#' => CellType::Wall,
            'P' => CellType::Partner,
            'D' => CellType::DishWasher,
            'W' => CellType::Window,
            'B' => CellType::Blueberries,
            'I' => CellType::IceCream,
            'S' => CellType::Strawberries,
            'H' => CellType::Dough,
            'C' => CellType::Chopper,
            'O' => CellType::Oven,
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
        self.cell_type == CellType::Empty
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
            if cell.is_visitable() {
                return 1;
            }
        }
        i32::MAX
    }

    pub fn reset(&mut self) {
        self.cells.clear();
    }

    /// get the 8 directions neighbors, this function can return less than 4 points if cells are not visitables or do no exists
    pub fn get_neighbors_points(&self, pos: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let points = [
            &Point::new(pos.x - 1, pos.y),
            &Point::new(pos.x - 1, pos.y - 1),
            &Point::new(pos.x - 1, pos.y + 1),
            &Point::new(pos.x + 1, pos.y),
            &Point::new(pos.x + 1, pos.y - 1),
            &Point::new(pos.x + 1, pos.y + 1),
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
                        let position = closed
                            .iter()
                            .position(|p| p.point == parent_point)
                            .unwrap_or_else(|| {
                                panic!(
                                    "WHAT ASTAR POINT NOT FOUND IN CLOSED QUEUE? {:?} | {:?}",
                                    parent_point, closed,
                                )
                            });
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum ItemType {
    Dish,
    Blueberries,
    IceCream,
    Strawberries,
    ChoppedStrawberries,
    Croissant,
    Dough,
}

impl ItemType {
    fn from_str(s: &str) -> Self {
        match s {
            "DISH" => return ItemType::Dish,
            "BLUEBERRIES" => return ItemType::Blueberries,
            "ICE_CREAM" => return ItemType::IceCream,
            "STRAWBERRIES" => return ItemType::Strawberries,
            "CHOPPED_STRAWBERRIES" => return ItemType::ChoppedStrawberries,
            "CROISSANT" => return ItemType::Croissant,
            "DOUGH" => return ItemType::Dough,
            _ => panic!("unknown item type: {}", s),
        }
    }

    fn decode_full_item(str: String) -> Vec<Self> {
        if str == "NONE" {
            return Vec::new();
        }
        str.split("-").map(|s| ItemType::from_str(s)).collect()
    }

    fn diff(a: Vec<Self>, b: Vec<Self>) -> Vec<Self> {
        let mut diff = Vec::new();
        for item_a in &a {
            if !b.contains(item_a) {
                diff.push(item_a.clone())
            }
        }
        for item_b in &b {
            if !a.contains(item_b) {
                diff.push(item_b.clone())
            }
        }
        diff
    }
}

#[derive(Debug)]
struct Oven {
    content: Option<ItemType>,
    timer: i32,
    pos: Point,
}

impl Oven {
    fn new(pos: Point) -> Self {
        Self {
            pos,
            content: None,
            timer: 0,
        }
    }

    fn update_from_raw(&mut self, str: String) {
        let inputs = str.split(" ").collect::<Vec<_>>();
        let oven_contents = inputs[0].trim().to_string(); // ignore until wood 1 league
        self.content = None;
        if oven_contents != "NONE" {
            self.content = Some(ItemType::from_str(&oven_contents));
        }
        self.timer = parse_input!(inputs[1], i32);
    }
}

#[derive(Debug, Clone)]
struct Command {
    id: i32,
    order: Vec<ItemType>,
    award: i32,
}

impl Command {
    fn decode(id: i32, str: String) -> Self {
        let inputs = str.split(" ").collect::<Vec<_>>();
        let customer_item = inputs[0].trim().to_string();
        let award = parse_input!(inputs[1], i32);
        let order = ItemType::decode_full_item(customer_item);

        Self { id, award, order }
    }
}

#[derive(Debug)]
struct Table {
    items: Vec<ItemType>,
    pos: Point,
}

impl Table {
    fn new(items: Vec<ItemType>, pos: Point) -> Self {
        Self { items, pos }
    }
}

struct Game {
    grid: Grid,
    windows: Vec<Point>,
    choppers: Vec<Point>,
    oven: Option<Oven>,
    crates: HashMap<ItemType, Point>,

    commands: Vec<Command>,
    tables: Vec<Table>,
    partner_pos: Option<Point>,
    partner_hand: Vec<ItemType>,

    player_pos: Point,
    player_hand: Vec<ItemType>,
    player_command_id: Option<i32>,
    player_chopping_item: Option<ItemType>,
    player_dropping_choped_item: bool,
    player_baking_item: Option<ItemType>,
    player_dropping_backed_item: bool,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            grid: Grid::new(w, h),
            commands: Vec::new(),
            player_pos: Point::new(0, 0),
            player_hand: Vec::new(),
            partner_pos: None,
            partner_hand: Vec::new(),
            tables: Vec::new(),
            player_command_id: None,
            windows: Vec::new(),
            crates: HashMap::new(),
            choppers: Vec::new(),
            oven: None,
            player_chopping_item: None,
            player_dropping_choped_item: false,
            player_baking_item: None,
            player_dropping_backed_item: false,
        }
    }

    pub fn is_oven_used(&self) -> bool {
        self.oven
            .as_ref()
            .map_or(false, |oven| oven.content.is_some())
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    pub fn start_new_loop(&mut self) {
        self.commands.clear();
        self.tables.clear();
    }

    pub fn set_player_pos(&mut self, player_pos: Point) {
        self.player_pos = player_pos;
    }

    pub fn set_player_hand(&mut self, hand: Vec<ItemType>) {
        self.player_hand = hand;
    }

    pub fn set_partner_pos(&mut self, partner_pos: Point) {
        if let Some(old_pos) = self.partner_pos {
            self.grid.set_cell_type(&old_pos, CellType::Empty);
        }
        self.partner_pos = Some(partner_pos.clone());
        self.grid.set_cell_type(&partner_pos, CellType::Partner);
    }

    pub fn set_partner_hand(&mut self, partner_hand: Vec<ItemType>) {
        self.partner_hand = partner_hand;
    }

    pub fn decode_row(&mut self, y: i32, row: String) {
        for (x, c) in row.chars().enumerate() {
            let cell_type = CellType::from_char(c);
            let pos = &Point::new(x as i32, y);
            self.grid.set_cell_type(pos, cell_type.clone());

            match cell_type {
                CellType::Window => {
                    self.windows.push(pos.clone());
                }
                CellType::Blueberries => {
                    self.crates.insert(ItemType::Blueberries, pos.clone());
                }
                CellType::IceCream => {
                    self.crates.insert(ItemType::IceCream, pos.clone());
                }
                CellType::DishWasher => {
                    self.crates.insert(ItemType::Dish, pos.clone());
                }
                CellType::Strawberries => {
                    self.crates.insert(ItemType::Strawberries, pos.clone());
                }
                CellType::Dough => {
                    self.crates.insert(ItemType::Dough, pos.clone());
                }
                CellType::Chopper => {
                    self.choppers.push(pos.clone());
                }
                CellType::Oven => {
                    self.oven = Some(Oven::new(pos.clone()));
                }
                _ => {}
            }
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn find_empty_space(&self) -> Option<Point> {
        for cell in &self.grid.cells {
            if cell.cell_type == CellType::Wall {
                let mut is_table = false;
                for table in &self.tables {
                    if table.pos == cell.pos {
                        is_table = true;
                    }
                }
                if !is_table {
                    return Some(cell.pos);
                }
            }
        }
        return None;
    }

    pub fn drop_choped_item(&mut self) -> String {
        eprintln!("-> choped! need to find a table to drop it now");
        let pos = self
            .find_empty_space()
            .expect("no empty space found to drop hand");
        self.player_dropping_choped_item = true;
        return format!("USE {} {}", pos.x, pos.y);
    }

    pub fn drop_baked_item(&mut self) -> String {
        eprintln!("-> baked! need to find a table to drop it now");
        let pos = self
            .find_empty_space()
            .expect("no empty space found to drop hand");
        self.player_dropping_backed_item = true;
        return format!("USE {} {}", pos.x, pos.y);
    }

    pub fn find_command_without_baking(&self) -> Option<Command> {
        for command in &self.commands {
            if command.order.contains(&ItemType::Croissant) {
                continue;
            }
            return Some(command.clone());
        }
        None
    }

    pub fn step(&mut self) -> String {
        // find a command
        if self.player_command_id.is_none() {
            // TODO: not started by partner
            eprintln!("-> no command picked yet, taking one");
            self.player_command_id = Some(0);
        }

        let player_command_id = self.player_command_id.expect("player do  not have command");
        let command = self
            .commands
            .get(player_command_id as usize)
            .unwrap_or_else(|| panic!("command id not found in vec: {}", player_command_id));

        eprintln!("player_hand: {:?}", self.player_hand);
        eprintln!("command_order: {:?}", command.order);

        // we finished baking, find a table
        if self.player_dropping_backed_item {
            if self.player_hand.len() == 0 {
                eprintln!("-> ok we have droped baked item - let resume");
                self.player_baking_item = None;
                self.player_dropping_backed_item = false;
            } else {
                return self.drop_baked_item();
            }
        }

        // if we are starting to bake
        if let Some(player_baking_item) = self.player_baking_item {
            if !self.is_oven_used() {
                eprintln!("-> need to bake item: {:?}", player_baking_item);
                if self.player_hand.len() > 1 || self.player_hand.contains(&ItemType::Dish) {
                    eprintln!("-> hands are full - need to drop hand somewhere");
                    let pos = self
                        .find_empty_space()
                        .expect("no empty space found to drop hand");
                    return format!("USE {} {}", pos.x, pos.y);
                }
                if self.player_hand.contains(&ItemType::Dough) {
                    eprintln!(
                        "-> already having item to bake - baking it: {:?}",
                        ItemType::Dough
                    );
                    let oven = &self.oven.as_ref().expect("no oven found!").pos;
                    return format!("USE {} {}", oven.x, oven.y);
                }
                if self.player_hand.contains(&ItemType::Croissant) {
                    return self.drop_baked_item();
                }

                eprintln!("-> not having item - finding one");
                return self.find_item(player_baking_item).unwrap_or("WAIT".into());
            } else {
                // if we have something in hand it means we get interrupteed
                // if this is not a plate, drop it somewhere and resume
                // in all cases we clean the need of baking for now
                self.player_baking_item = None;

                // finding a command without baking need and resume from here
                if let Some(command) = self.find_command_without_baking() {
                    self.player_command_id = Some(command.id);
                }

                if self.player_hand.len() > 1 || !self.player_hand.contains(&ItemType::Dish) {
                    eprintln!(
                        "-> we wanted to bake but got interrupted -> dropping item to resume"
                    );
                    return self.drop_baked_item();
                }

                return self.step();
            }
        }

        // we finished choping, finding a table
        if self.player_dropping_choped_item {
            if self.player_hand.len() == 0 {
                eprintln!("-> ok we have droped choped item - let resume");
                self.player_chopping_item = None;
                self.player_dropping_choped_item = false;
            } else {
                return self.drop_choped_item();
            }
        }

        // if we are starting to chop, chop
        if let Some(player_chopping_item) = self.player_chopping_item {
            eprintln!("-> need to chope item: {:?}", player_chopping_item);
            if self.player_hand.len() > 1 || self.player_hand.contains(&ItemType::Dish) {
                eprintln!("-> hands are full - need to drop hand somewhere");
                let pos = self
                    .find_empty_space()
                    .expect("no empty space found to drop hand");
                return format!("USE {} {}", pos.x, pos.y);
            }
            if self.player_hand.contains(&ItemType::Strawberries) {
                eprintln!(
                    "-> already having item to chope - chopping it: {:?}",
                    ItemType::Strawberries
                );
                let choper = self.choppers.get(0).expect("no choper found!");
                return format!("USE {} {}", choper.x, choper.y);
            }
            if self.player_hand.contains(&ItemType::ChoppedStrawberries) {
                return self.drop_choped_item();
            }

            eprintln!("-> not having item - finding one");
            return self
                .find_item(player_chopping_item)
                .unwrap_or("WAIT".into());
        }

        // command complete, deliver it
        let mut diff = ItemType::diff(self.player_hand.clone(), command.order.clone());
        if diff.len() == 0 {
            eprintln!("-> command is completed, delivering it");
            // find window and go there
            self.player_hand.clear();
            return format!("USE {} {}", self.windows[0].x, self.windows[0].y);
        }

        // command imcomplete, complete it
        // TODO: find closest item -and first dish-
        eprintln!("-> command is incomplete, find missing elements starting with dish");
        let mut missing_item = ItemType::Dish;
        if !diff.contains(&ItemType::Dish) {
            eprintln!("-> dish is there we need food now");
            missing_item = diff
                .pop()
                .expect("Diff order is empty, but should not at this stage");
        }

        // - Ok we may need to chope or bake
        if let Some(action) = self.find_item(missing_item) {
            return action;
        }
        if missing_item == ItemType::ChoppedStrawberries {
            self.player_chopping_item = Some(ItemType::Strawberries);
            return self.step();
        }
        if missing_item == ItemType::Croissant {
            if self.is_oven_used() {
                if let Some(next_missing) = diff.pop() {
                    return self.find_item(next_missing).unwrap_or("WAIT".into());
                } else {
                    return "WAIT".into();
                }
            }
            self.player_baking_item = Some(ItemType::Dough);
            return self.step();
        }

        return self.find_item(missing_item).unwrap_or("WAIT".into());
    }

    fn find_item(&self, missing_item: ItemType) -> Option<String> {
        // - Table ?
        eprintln!("-> finding a table with item {:?}", missing_item);
        for table in &self.tables {
            if table.items.contains(&missing_item) {
                eprintln!("-> table contains the items: {:?}", table);
                // TODO:
                if table.items.len() > 1 {
                    eprintln!(
                        "-> this table contains items but with other, skipping this table for now"
                    );
                    continue;
                }

                return Some(format!("USE {} {}", table.pos.x, table.pos.y));
            }
        }
        // - Crate then
        eprintln!("-> not finding a table with item, going into crate");
        if let Some(crate_pos) = self.crates.get(&missing_item) {
            return Some(format!("USE {} {}", crate_pos.x, crate_pos.y));
        }

        eprintln!("-> not finding item");
        return None;
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_all_customers = parse_input!(input_line, i32);
    let mut game = Game::new(11, 7);
    for i in 0..num_all_customers as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let customer_item = inputs[0].trim().to_string(); // the food the customer is waiting for
        let customer_award = parse_input!(inputs[1], i32); // the number of points awarded for delivering the food
    }
    for y in 0..7 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let kitchen_line = input_line.trim_matches('\n').to_string();
        game.decode_row(y as i32, kitchen_line);
    }

    // game loop
    loop {
        game.start_new_loop();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let turns_remaining = parse_input!(input_line, i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let player_x = parse_input!(inputs[0], i32);
        let player_y = parse_input!(inputs[1], i32);
        game.set_player_pos(Point::new(player_x, player_y));
        let player_item = inputs[2].trim().to_string();
        game.set_player_hand(ItemType::decode_full_item(player_item));
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let partner_x = parse_input!(inputs[0], i32);
        let partner_y = parse_input!(inputs[1], i32);
        game.set_partner_pos(Point::new(partner_x, partner_y));
        let partner_item = inputs[2].trim().to_string();
        game.set_partner_hand(ItemType::decode_full_item(partner_item));
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_tables_with_items = parse_input!(input_line, i32); // the number of tables in the kitchen that currently hold an item
        for _ in 0..num_tables_with_items as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let table_x = parse_input!(inputs[0], i32);
            let table_y = parse_input!(inputs[1], i32);
            let item = inputs[2].trim().to_string();
            let items = ItemType::decode_full_item(item);
            game.add_table(Table::new(items, Point::new(table_x, table_y)))
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        if let Some(oven) = &mut game.oven {
            oven.update_from_raw(input_line);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_customers = parse_input!(input_line, i32); // the number of customers currently waiting for food
        for i in 0..num_customers as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // let inputs = input_line.split(" ").collect::<Vec<_>>();
            // let customer_item = inputs[0].trim().to_string();
            // let customer_award = parse_input!(inputs[1], i32);
            eprintln!("adding a command: {}", input_line);
            game.add_command(Command::decode(i as i32, input_line))
        }

        eprintln!("{:?}", game.grid);
        eprintln!("{:?}", &game.oven);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // MOVE x y
        // USE x y
        // WAIT
        println!("{}", game.step());
    }
}
