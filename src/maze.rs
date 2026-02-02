use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::hash::{Hash, Hasher};

use strum::IntoEnumIterator;

use crate::singlemaze::{Direction, SingleMaze, SingleMazeState};

#[derive(Clone)]
pub struct MazeState {
    maze_one_state: SingleMazeState,
    maze_two_state: SingleMazeState,
    solution: Vec<Direction>,
}

impl Hash for MazeState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.maze_one_state.hash(state);
        self.maze_two_state.hash(state);
    }
}

impl PartialEq for MazeState {
    fn eq(&self, other: &Self) -> bool {
        self.maze_one_state == other.maze_one_state && self.maze_two_state == other.maze_two_state
    }
}

impl Eq for MazeState {}

impl MazeState {
    fn won(&self) -> bool {
        self.maze_one_state.robot_outside && self.maze_two_state.robot_outside
    }
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
        let maze_state = MazeState {maze_one_state, maze_two_state, solution: vec![]};
        (maze, maze_state)
    }

    fn solve(&self, state: MazeState) -> Vec<Direction> {
        if self.no_exits_in_a_maze {
            return vec![];
        }
        let mut hashes_seen = HashSet::new();
        hashes_seen.insert(state.clone());
        let mut to_explore_next = vec![state.clone()];
        for _ in 0..1000 {
            if to_explore_next.len() == 0 {break;}
            let to_explore = to_explore_next.clone();
            to_explore_next = vec![];
            for maze_state in to_explore.into_iter() {
                for direction in Direction::iter() {
                    let (allowed, new_state) = self.step(&maze_state, &direction);
                    if allowed {
                        if new_state.won() {
                            return new_state.solution;
                        }
                        else if hashes_seen.insert(new_state.clone()) {
                            to_explore_next.push(new_state);
                        }
                    }
                }
            }
        }
        vec![]
    }

    fn step(&self, state: &MazeState, direction: &Direction) -> (bool, MazeState) {
        let mut solution = state.solution.clone();
        solution.push(direction.clone());
        let (one, maze_one_state) = self.maze_one.step(&state.maze_one_state, direction);
        let (two, maze_two_state) = self.maze_two.step(&state.maze_two_state, direction);
        let new_state = MazeState {maze_one_state, maze_two_state, solution};
        (one && two, new_state)
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
