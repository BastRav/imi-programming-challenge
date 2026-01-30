use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::singlemaze::{Direction, SingleMaze};

#[derive(Clone)]
pub struct Maze {
    maze_one: SingleMaze,
    maze_two: SingleMaze,
    solution: Vec<Direction>,
}

impl Maze {
    pub fn new<P>(path: P) -> Self
    where P: AsRef<Path> {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
        let maze_one = SingleMaze::from_lines(&mut lines);
        let maze_two = SingleMaze::from_lines(&mut lines);
        Maze { maze_one, maze_two, solution: vec![]}
    }

    fn solve(&mut self) {
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
                    else {
                        to_explore_next.push(maze1);
                    }
                }
                let mut maze2 = maze.clone();
                if maze2.step(&Direction::North) {
                    if maze2.won() {
                        self.solution = maze2.solution;
                        return;
                    }
                    else {
                        to_explore_next.push(maze2);
                    }
                }
                let mut maze3 = maze.clone();
                if maze3.step(&Direction::South) {
                    if maze3.won() {
                        self.solution = maze3.solution;
                        return;
                    }
                    else {
                        to_explore_next.push(maze3);
                    }
                }
                if maze.step(&Direction::West) {
                    if maze.won() {
                        self.solution = maze.solution;
                        return;
                    }
                    else {
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
