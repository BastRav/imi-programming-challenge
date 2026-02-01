use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::hash::{Hash, DefaultHasher, Hasher};
use std::vec;

use strum::IntoEnumIterator;

use crate::singlemaze::{Direction, SingleMaze, SingleMazeState};

#[derive(Clone, Debug)]
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

impl MazeState {
    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub struct Maze {
    maze_one: SingleMaze,
    maze_two: SingleMaze,
}

impl Maze {
    pub fn new<P>(path: P) -> (Self, MazeState)
    where P: AsRef<Path> {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
        let (maze_one, maze_one_state) = SingleMaze::from_lines(&mut lines);
        let (maze_two, maze_two_state) = SingleMaze::from_lines(&mut lines);
        let maze = Maze { maze_one, maze_two};
        let maze_state = MazeState {maze_one_state, maze_two_state, solution: vec![],};
        (maze, maze_state)
    }

    fn solve(&mut self, state: &MazeState) -> Vec<Direction> {
        let mut hashes_seen = HashSet::new();
        hashes_seen.insert(state.get_hash());
        let mut to_explore_next = vec![state.clone()];
        let mut best_best_case = usize::MAX;
        let mut best_sum = usize::MAX;
        for _ in 0..1000 {
            // println!("{:#?}", to_explore_next);
            if to_explore_next.len() == 0 {
                break;
            }
            let to_explore = to_explore_next.clone();
            to_explore_next = vec![];
            for maze_state in to_explore.into_iter() {
                let next_moves_one = self.maze_one.next_moves(&maze_state.maze_one_state);
                let next_moves_two = self.maze_two.next_moves(&maze_state.maze_two_state);
                // println!("{:#?}", next_moves_one.iter().map(|n| (n.0, n.1.0)).collect::<Vec<(&Direction, bool)>>());
                for direction in Direction::iter() {
                    match next_moves_one.get(&direction) {
                        Some(state_one) => {
                            match next_moves_two.get(&direction) {
                                Some(state_two) => {
                                    let best_case = state_one.depth_to_solution.max(state_two.depth_to_solution);
                                    let sum = state_one.depth_to_solution + state_two.depth_to_solution;
                                    if best_case < best_best_case {
                                        best_best_case = best_case;
                                    }
                                    if sum < best_sum {
                                        best_sum = sum;
                                    }
                                    if best_case <= best_best_case + 2 && sum <= best_sum + 2 {
                                        let mut solution = maze_state.solution.clone();
                                        solution.push(direction);
                                        let new_state = MazeState {
                                            maze_one_state: state_one.clone(),
                                            maze_two_state: state_two.clone(),
                                            solution,
                                        };
                                        if sum == 0 {
                                            return new_state.solution.clone();
                                        }
                                        to_explore_next.push(new_state);
                                    }
                                },
                                None => (),
                            }
                        },
                        None => (),
                    }
                }
            }
        }
        vec![]
    }

    pub fn write_solution<P>(&mut self, state:&MazeState, output_path: P) -> std::io::Result<()>
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
