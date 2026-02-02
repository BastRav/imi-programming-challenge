use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use strum::IntoEnumIterator;

use crate::singlemaze::{Direction, SingleMaze, SingleMazeState};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct MazeState {
    maze_one_state: SingleMazeState,
    maze_two_state: SingleMazeState,
}

impl MazeState {
    fn won(&self) -> bool {
        self.maze_one_state.robot_outside && self.maze_two_state.robot_outside
    }
}

#[derive(Clone)]
struct Node {
    maze_state: MazeState,
    solution: Vec<Direction>,
}

pub struct Maze {
    maze_one: SingleMaze,
    maze_two: SingleMaze,
    no_exits_in_a_maze: bool,
}

impl Maze {
    pub fn new<P>(path: P) -> (Self, MazeState)
    where P: AsRef<Path> {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
        let (maze_one, maze_one_state) = SingleMaze::from_lines(&mut lines);
        let (maze_two, maze_two_state) = SingleMaze::from_lines(&mut lines);
        let no_exits_in_a_maze = maze_one.no_exit() || maze_two.no_exit();
        let maze = Maze { maze_one, maze_two, no_exits_in_a_maze };
        let maze_state = MazeState {maze_one_state, maze_two_state};
        (maze, maze_state)
    }

    fn solve(&self, state: MazeState) -> Vec<Direction> {
        if self.no_exits_in_a_maze {
            return vec![];
        }
        let mut seen = HashSet::with_capacity(100_000);
        seen.insert(state.clone());
        let mut to_explore_next = Vec::with_capacity(10_000);
        to_explore_next.push( Node {
            maze_state: state.clone(),
            solution: Vec::with_capacity(1000),
        });
        for _ in 0..1000 {
            if to_explore_next.len() == 0 {break;}
            let to_explore = std::mem::take(&mut to_explore_next);
            for node in to_explore.into_iter() {
                for direction in Direction::iter() {
                    let (allowed, new_node) = self.step(&node, &direction);
                    if allowed {
                        if new_node.maze_state.won() {
                            return new_node.solution;
                        }
                        else if seen.insert(new_node.maze_state.clone()) {
                            to_explore_next.push(new_node);
                        }
                    }
                }
            }
        }
        vec![]
    }

    fn step(&self, node: &Node, direction: &Direction) -> (bool, Node) {
        let mut solution = node.solution.clone();
        solution.push(direction.clone());
        let (one, maze_one_state) = self.maze_one.step(&node.maze_state.maze_one_state, direction);
        let (two, maze_two_state) = self.maze_two.step(&node.maze_state.maze_two_state, direction);
        let new_node = Node {
            maze_state: MazeState {maze_one_state, maze_two_state},
            solution,
        };
        (one && two, new_node)
    }

    pub fn write_solution<P>(&self, state: MazeState, output_path: P) -> std::io::Result<()>
    where P: AsRef<Path> {
        let solution = self.solve(state);
        let mut output_file = File::create(output_path)?;
        let solution_length = solution.len();
        if solution_length == 0 {
            output_file.write_all("-1".to_string().as_bytes())?;
        }
        else {
            let mut line_one = solution_length.to_string();
            line_one.push_str("\n");
            output_file.write_all(line_one.as_bytes())?;
            let solution_vec: Vec<String> = solution.iter().map(|d| d.to_char().to_string()).collect();
            let solution = solution_vec.join("\n");
            output_file.write_all(solution.as_bytes())?;
        }
        Ok(())
    }
}
