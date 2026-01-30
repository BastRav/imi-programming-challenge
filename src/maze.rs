use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::singlemaze::{Direction, SingleMaze};

#[derive(Clone)]
pub struct Maze {
    maze_one: SingleMaze,
    maze_two: SingleMaze,
    solution: Vec<Direction>,
    guards_cycle: u8,
}

impl Maze {
    pub fn new<P>(path: P) -> Self
    where P: AsRef<Path> {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
        let maze_one = SingleMaze::from_lines(&mut lines);
        let maze_two = SingleMaze::from_lines(&mut lines);
        Maze { maze_one, maze_two, solution: vec![], guards_cycle: 0}
    }

    fn get_hash(&self) -> u32 {
        let maze_one_hash = self.maze_one.get_hash(); // 1st to 10th for 1st maze
        let maze_two_hash = self.maze_two.get_hash() << 10; // 11th to 20th for 2nd maze
        let guards_cycle_hash = (self.guards_cycle as u32) << 20; // guards cycles in maximum 24 steps -> 21st to 25th
        return maze_one_hash + maze_two_hash + guards_cycle_hash
    }

    fn solve(&mut self) {
        let mut hashes_seen = HashSet::new();
        hashes_seen.insert(self.get_hash());
        let mut to_explore_next = vec![self.clone()];
        for _ in 0..1000 {
            let to_explore = to_explore_next.clone();
            to_explore_next = vec![];
            for mut maze in to_explore.into_iter() {
                let mut maze1 = maze.clone();
                if maze1.step(&Direction::East) {
                    if maze1.won() {
                        self.solution = maze1.solution;
                        return;
                    }
                    else if hashes_seen.insert(maze1.get_hash()) {
                        to_explore_next.push(maze1);
                    }
                }
                let mut maze2 = maze.clone();
                if maze2.step(&Direction::North) {
                    if maze2.won() {
                        self.solution = maze2.solution;
                        return;
                    }
                    else if hashes_seen.insert(maze2.get_hash()){
                        to_explore_next.push(maze2);
                    }
                }
                let mut maze3 = maze.clone();
                if maze3.step(&Direction::South) {
                    if maze3.won() {
                        self.solution = maze3.solution;
                        return;
                    }
                    else if hashes_seen.insert(maze3.get_hash()){
                        to_explore_next.push(maze3);
                    }
                }
                if maze.step(&Direction::West) {
                    if maze.won() {
                        self.solution = maze.solution;
                        return;
                    }
                    else if hashes_seen.insert(maze.get_hash()){
                        to_explore_next.push(maze);
                    }
                }
            }
        }
    }

    fn step(&mut self, direction: &Direction) -> bool {
        let one = self.maze_one.step(direction);
        let two = self.maze_two.step(direction);
        self.solution.push(direction.clone());
        self.guards_cycle += 1;
        if self.guards_cycle == 24 {self.guards_cycle = 0;}
        one && two
    }

    fn won(&self) -> bool {
        self.maze_one.robot_outside && self.maze_two.robot_outside
    }

    pub fn write_solution<P>(&mut self, output_path: P) -> std::io::Result<()>
    where P: AsRef<Path> {
        self.solve();
        let mut output_file = File::create(output_path)?;
        let solution_length = self.solution.len();
        if solution_length == 0 {
            output_file.write_all("-1".to_string().as_bytes())?;
        }
        else {
            let mut line_one = solution_length.to_string();
            line_one.push_str("\n");
            output_file.write_all(line_one.as_bytes())?;
            let solution_vec: Vec<String> = self.solution.iter().map(|d| d.to_char().to_string()).collect();
            let solution = solution_vec.join("\n");
            output_file.write_all(solution.as_bytes())?;
        }
        Ok(())
    }
}
