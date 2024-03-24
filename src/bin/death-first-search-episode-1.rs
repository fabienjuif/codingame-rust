// https://www.codingame.com/ide/puzzle/death-first-search-episode-1

use std::{
    collections::{HashMap, HashSet},
    io,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct Node {
    pub id: i32,
    pub links: Vec<i32>,
    pub is_exit: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: -1,
            links: Vec::new(),
            is_exit: false,
        }
    }
}

impl Node {
    pub fn add_link(&mut self, n: i32) {
        self.links.push(n);
    }

    pub fn remove_link(&mut self, n: i32) {
        if let Some(pos) = self.links.iter().position(|x| *x == n) {
            self.links.swap_remove(pos);
        }
    }
}
struct Game {
    pub grid: HashMap<i32, Node>,
    pub exits: Vec<i32>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            grid: HashMap::new(),
            exits: Vec::new(),
        }
    }
}

impl Game {
    pub fn create_empty_nodes(&mut self, n: i32) {
        for i in 0..n {
            self.grid.insert(
                i,
                Node {
                    id: i,
                    ..Default::default()
                },
            );
        }
    }

    pub fn add_link(&mut self, n1: i32, n2: i32) {
        self.grid.get_mut(&n1).unwrap().add_link(n2);
        self.grid.get_mut(&n2).unwrap().add_link(n1);
    }

    pub fn remove_link(&mut self, n1: i32, n2: i32) {
        self.grid.get_mut(&n1).unwrap().remove_link(n2);
        self.grid.get_mut(&n2).unwrap().remove_link(n1);
    }

    pub fn add_exit(&mut self, n: i32) {
        self.exits.push(n);
        if let Some(node) = self.get_mut_node(&n) {
            node.is_exit = true;
        }
    }

    pub fn get_node(&self, n: &i32) -> Option<&Node> {
        self.grid.get(n)
    }

    pub fn get_mut_node(&mut self, n: &i32) -> Option<&mut Node> {
        self.grid.get_mut(n)
    }

    pub fn get_linked_nodes(&self, n: i32) -> Vec<&Node> {
        let mut res = Vec::<&Node>::new();
        if let Some(node) = self.grid.get(&n) {
            for l in &node.links {
                if let Some(node) = self.get_node(l) {
                    res.push(node);
                }
            }
        }

        res
    }

    /// returns the link (starting and ending nodes ids)
    pub fn find_closest_node_toward_exit(&self, start: &i32) -> Option<(i32, i32)> {
        struct NodeWithParent<'a> {
            node: &'a Node,
            parent_id: i32,
        }
        let mut next_nodes = Vec::<NodeWithParent>::new();
        let mut already_seen_nodes = HashSet::<i32>::new();
        let Some(starting_node) = self.get_node(start) else {
            return None;
        };
        next_nodes.push(NodeWithParent {
            node: starting_node,
            parent_id: *start,
        });
        loop {
            let Some(nwp) = next_nodes.pop() else {
                return None;
            };
            already_seen_nodes.insert(nwp.node.id);
            if nwp.node.is_exit {
                return Some((nwp.parent_id, nwp.node.id));
            }
            let mut links = self.get_linked_nodes(nwp.node.id);
            links.retain(|l| !already_seen_nodes.contains(&l.id));
            let mut links_with_parent = Vec::<NodeWithParent>::with_capacity(links.len());
            for l in links {
                links_with_parent.push(NodeWithParent {
                    node: l,
                    parent_id: nwp.node.id,
                })
            }
            next_nodes.splice(0..0, links_with_parent);
            // for l in self.get_linked_nodes(nwp.node.id) {
            //     if !already_seen_nodes.contains(&l.id) {
            //         next_nodes(NodeWithParent {
            //             node: l,
            //             parent_id: nwp.node.id,
            //         });
            //     }
            // }
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut game = Game::default();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let n = parse_input!(inputs[0], i32); // the total number of nodes in the level, including the gateways
    let l = parse_input!(inputs[1], i32); // the number of links
    let e = parse_input!(inputs[2], i32); // the number of exit gateways
    game.create_empty_nodes(n);

    for _ in 0..l as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let n1 = parse_input!(inputs[0], i32); // N1 and N2 defines a link between these nodes
        let n2 = parse_input!(inputs[1], i32);
        game.add_link(n1, n2);
    }
    for _ in 0..e as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let ei = parse_input!(input_line, i32); // the index of a gateway node
        game.add_exit(ei);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let si = parse_input!(input_line, i32); // The index of the node on which the Bobnet agent is positioned this turn

        if let Some((l, r)) = game.find_closest_node_toward_exit(&si) {
            game.remove_link(l, r);
            println!("{} {}", l, r);
        } else {
            eprintln!("WHAT");
            println!("sorry");
            panic!("should not happen")
        }
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // Example: 0 1 are the indices of the nodes you wish to sever the link between
        // println!("0 1");
    }
}
