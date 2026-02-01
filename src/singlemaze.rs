use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};

use petgraph::Direction::Incoming;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use petgraph::{graph::{DiGraph, NodeIndex}, visit::EdgeRef};

#[derive(EnumIter, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(input_char: char) -> Self {
        match input_char {
            'N' => Self::North,
            'E' => Self::East,
            'S' => Self::South,
            'W' => Self::West,
            _ => panic!("Invalid input string for Direction"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::North => 'N',
            Self::East => 'E',
            Self::South => 'S',
            Self::West => 'W',
        }
    }

    fn to_position_change(&self, columns: usize) -> isize {
        match self {
            Self::North => -(columns as isize),
            Self::East => 1,
            Self::South => columns as isize,
            Self::West => -1,
        }
    }
}

#[derive(Clone, Debug)]
struct GuardState {
    position: usize,
    reversed_direction: bool,
    steps_to_starting_position: usize,
}

impl Hash for GuardState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.steps_to_starting_position.hash(state);
    }
}

impl PartialEq for GuardState {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.reversed_direction == other.reversed_direction
    }
}

pub struct Guard {
    patrol_path_size: usize,
    movement: isize,
}

impl Guard {
    fn new(patrol_path_size: usize, movement: isize) -> Self {
        Self {
            patrol_path_size,
            movement,
        }
    }

    fn step(&self, state: &GuardState) -> GuardState {
        let mut new_state = state.clone();
        if state.steps_to_starting_position == 0 {
            new_state.reversed_direction = false;
        }
        else if state.steps_to_starting_position == self.patrol_path_size - 1 {
            new_state.reversed_direction = true;
        }
        let mut position_isize = new_state.position as isize;
        if new_state.reversed_direction {
            new_state.steps_to_starting_position -= 1;
            position_isize -= self.movement;
        }
        else {
            new_state.steps_to_starting_position += 1;
            position_isize += self.movement;
        }
        new_state.position = position_isize as usize;
        new_state
    }
}

#[derive(Clone, Debug)]
pub struct SingleMazeState {
    guards_states: Vec<GuardState>,
    robot_position: usize,
    pub robot_outside: bool,
    solutions: HashSet<Direction>,
    solutions_computed: bool,
    depth: usize,
    depth_to_solution: usize,
}

impl Hash for SingleMazeState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.guards_states.hash(state);
        self.robot_position.hash(state);
        self.robot_outside.hash(state);
    }
}

impl PartialEq for SingleMazeState {
    fn eq(&self, other: &Self) -> bool {
        self.guards_states == other.guards_states && self.robot_position == other.robot_position && self.robot_outside == other.robot_outside
    }
}

impl Eq for SingleMazeState {}

impl SingleMazeState {
    fn new(guards_states: Vec<GuardState>, robot_position: usize, robot_outside: bool, depth: usize) -> Self {
        Self {
            guards_states,
            robot_position,
            robot_outside,
            solutions: HashSet::new(),
            solutions_computed: false,
            depth,
            depth_to_solution: 0,
        }
    }
}

pub struct SingleMaze {
    columns: usize,
    layout: Vec<bool>, // false means wall, true means open
    guards: Vec<Guard>,
    exits: Vec<(usize, Direction)>,
    hashes_seen: HashMap<SingleMazeState, NodeIndex>,
    graph: DiGraph<SingleMazeState, Direction>,
}

impl SingleMaze {
    pub fn from_lines(lines: &mut impl Iterator<Item = String>) -> (Self, SingleMazeState) {
        let line_one = lines.next().unwrap();
        let mut line_one_split = line_one.split(' ').map(|n| n.parse::<usize>().unwrap());
        let rows = line_one_split.next().unwrap();
        let columns = line_one_split.next().unwrap();
        let mut layout = vec![];
        let mut initial_position = 0;
        let mut exits = vec![];
        for row in 0..rows {
            for (column, char) in lines.next().unwrap().chars().enumerate() {
                match char {
                    '#' => layout.push(false),
                    '.' => {
                        layout.push(true);
                        let position = row * columns + column;
                        if row == 0 {exits.push((position, Direction::North));}
                        if row == rows-1 {exits.push((position, Direction::South));}
                        if column == 0 {exits.push((position, Direction::West));}
                        if column == columns-1 {exits.push((position, Direction::East));}
                    },
                    'X' => {
                        let position = row * columns + column;
                        initial_position = position;
                        layout.push(true);
                        if row == 0 {exits.push((position, Direction::North));}
                        if row == rows-1 {exits.push((position, Direction::South));}
                        if column == 0 {exits.push((position, Direction::West));}
                        if column == columns-1 {exits.push((position, Direction::East));}
                    },
                    _ => panic!("Unexpected character"),
                }
            }
        }
        let number_guards = lines.next().unwrap().parse::<usize>().unwrap();
        let mut guards = vec![];
        let mut guards_states = vec![];
        for _ in 0..number_guards {
            let line_guard = lines.next().unwrap();
            let mut line_guard_split = line_guard.split(' ');
            let row = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let column = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let patrol_path_size = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let direction_str = line_guard_split.next().unwrap();
            let direction = Direction::from_char(direction_str.chars().next().unwrap());
            let guard = Guard::new(patrol_path_size, direction.to_position_change(columns));
            let state = GuardState {
                position: row * columns + column,
                reversed_direction: false,
                steps_to_starting_position: 0,
            };
            guards.push(guard);
            guards_states.push(state);
        }
        let state = SingleMazeState::new(guards_states, initial_position,false, 0);
        let mut hashes_seen = HashMap::new();
        let mut graph = DiGraph::new();
        let origin = graph.add_node(state.clone());
        hashes_seen.insert(state.clone(), origin);
        let maze = SingleMaze {
            columns: columns,
            layout: layout,
            guards,
            exits,
            hashes_seen,
            graph,
        };
        (maze, state)
    }

    fn solve(&mut self, node_index: NodeIndex) {
        let mut to_explore_next = vec![node_index];
        let mut depth = self.graph.node_weight(node_index).unwrap().depth;
        let mut already_seen_this_time = HashSet::new();
        already_seen_this_time.insert(node_index);
        let mut solutions_pending: Vec<(usize, NodeIndex)> = vec![];
        for _ in 0..1000 {
            depth += 1;
            if to_explore_next.len() == 0 {break;}
            let to_explore = to_explore_next.clone();
            to_explore_next = vec![];
            let mut new_graph = self.graph.clone();
            let mut solutions_found = vec![];
            for solution_pending in solutions_pending.iter_mut(){
                solution_pending.0 -= 1;
                if solution_pending.0 == 0 {
                    solutions_found.push(solution_pending.1);
                }
            }
            for node_index in to_explore.into_iter() {
                let maze_state = self.graph.node_weight(node_index).unwrap();
                if maze_state.solutions_computed {
                    if maze_state.depth_to_solution == 0 {
                        solutions_found.push(node_index);
                    }
                    else {
                        solutions_pending.push((maze_state.depth_to_solution, node_index));
                    }
                }
                else {
                    for direction in Direction::iter() {
                        let (allowed, new_state) = self.step(maze_state, &direction);
                        if allowed {
                            let new_index;
                            let already_seen = self.hashes_seen.contains_key(&new_state);
                            let mut already_seen_this = true;
                            if already_seen {
                                new_index = self.hashes_seen[&new_state];
                            }
                            else {
                                new_index = new_graph.add_node(new_state.clone());
                                self.hashes_seen.insert(new_state.clone(), new_index);
                            }
                            if already_seen_this_time.insert(new_index) {
                                // first time we see this, in the current search
                                // update depth, not the same starting point
                                let node_in_new_graph = new_graph.node_weight_mut(new_index).unwrap();
                                node_in_new_graph.depth = depth;
                                already_seen_this = false;
                            }
                            let mut edge_exists = false;
                            for edge_ref in new_graph.edges_connecting(node_index, new_index) {
                                if edge_ref.weight() == &direction {
                                    edge_exists = true;
                                    break;
                                }
                            }
                            if !edge_exists {
                                new_graph.add_edge(node_index, new_index, direction);
                            }
                            if new_state.robot_outside {
                                solutions_found.push(new_index);
                                let new_node = new_graph.node_weight_mut(new_index).unwrap();
                                new_node.solutions_computed = true;
                                new_node.depth_to_solution = 0;
                                for direction in Direction::iter() {
                                    new_node.solutions.insert(direction);
                                }
                            }
                            else if !already_seen_this {
                                to_explore_next.push(new_index);
                            }
                        }
                    }
                }
            }
            self.graph = new_graph;
            if solutions_found.len() > 0 {
                // println!("Solution found");
                let mut new_graph = self.graph.clone();
                for solution in solutions_found.into_iter() {
                    let mut nodes_to_visit_next = HashSet::new();
                    nodes_to_visit_next.insert(solution);
                    loop {
                        let nodes_to_visit = nodes_to_visit_next.clone();
                        nodes_to_visit_next = HashSet::new();
                        for node in nodes_to_visit.into_iter() {
                            let node_weight = self.graph.node_weight(node).unwrap();
                            let node_depth = node_weight.depth;
                            let node_depth_to_solution = node_weight.depth_to_solution;
                            for edge_ref in self.graph.edges_directed(node, Incoming){
                                let parent_node_index = edge_ref.source();
                                let parent_node = new_graph.node_weight_mut(parent_node_index).unwrap();
                                if parent_node.depth < node_depth {
                                    parent_node.solutions_computed = true;
                                    parent_node.depth_to_solution = node_depth_to_solution + 1;
                                    parent_node.solutions.insert(edge_ref.weight().clone());
                                    nodes_to_visit_next.insert(parent_node_index);
                                }
                            }
                        }
                        if nodes_to_visit_next.len() == 0 {break; }
                    }
                }
                self.graph = new_graph;
                break;
            }
        }
    }

    pub fn next_moves(&mut self, state: &SingleMazeState) -> HashMap<Direction, (bool, SingleMazeState)> {
        let mut next_moves = HashMap::new();
        let node_index = self.hashes_seen[state];
        let node = self.graph.node_weight_mut(node_index).unwrap();
        if node.robot_outside {
            // edges cannot be used here
            for direction in Direction::iter(){
                next_moves.insert(direction, (true, node.clone()));
            }
            return next_moves;
        }
        if !node.solutions_computed {
            node.depth = 0;
            self.solve(node_index);
        }
        let same_node = self.graph.node_weight(node_index).unwrap();
        if same_node.solutions.len() == 0 {
            // no solution, don't bother
            return next_moves;
        }
        for edge_ref in self.graph.edges(node_index) {
            let direction = edge_ref.weight();
            let new_node_index = edge_ref.target();
            let new_state = self.graph.node_weight(new_node_index).unwrap();
            let mut is_best = false;
            for best_direction in same_node.solutions.iter() {
                if best_direction == direction {
                    is_best = true;
                    break;
                }
            }
            next_moves.insert(direction.clone(), (is_best, new_state.clone()));
        }
        next_moves
    }

    fn step(&self, state: &SingleMazeState, direction: &Direction) -> (bool, SingleMazeState) {
        // never call this method if robot is already outside!!!
        let mut new_state = state.clone();
        new_state.depth += 1;
        for (position_exit, direction_exit) in self.exits.iter() {
            if state.robot_position == *position_exit && direction == direction_exit {
                // it's a win
                new_state.robot_outside = true;
                return (true, new_state);
            }
        }
        let robot_move = direction.to_position_change(self.columns);
        let new_robot_position = (state.robot_position as isize + robot_move) as usize;
        for (index_guard, guard) in self.guards.iter().enumerate(){
            let guard_state = &state.guards_states[index_guard];
            let new_guard_state = guard.step(guard_state);
            if new_guard_state.position == new_robot_position || (guard_state.position == new_robot_position && new_guard_state.position == state.robot_position) {
                // caught by a guard
                return (false, new_state);
            }
            new_state.guards_states[index_guard] = new_guard_state;
        }
        if self.layout[new_robot_position] {
            // move is possible
            new_state.robot_position = new_robot_position;
            return (true, new_state);
        }
        else {
            // hit a wall, it's allowed but you do not move
            return (true, new_state);
        }
    }
}
