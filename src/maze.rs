use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Clone)]
enum Direction {
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

    fn reverse(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self:: East,
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

struct Guard {
    starting_position: usize,
    patrol_path_size: usize,
    initial_direction: Direction,
    steps_to_starting_position: usize,
    reversed_direction: bool,
}

impl Guard {
    fn new(starting_position: usize, patrol_path_size: usize, initial_direction: Direction) -> Self {
        Self {
            starting_position,
            patrol_path_size,
            initial_direction,
            steps_to_starting_position: 0,
            reversed_direction: false,
        }
    }

    fn step(&mut self) {
        if self.patrol_path_size > 1 {
            if self.steps_to_starting_position == 0 {
                self.reversed_direction = false;
            }
            else if self.steps_to_starting_position == self.patrol_path_size - 1 {
                self.reversed_direction = true;
            }
            if self.reversed_direction {
                self.steps_to_starting_position -= 1;
            }
            else {
                self.steps_to_starting_position += 1;
            }
        }
    }
}

struct SingleMaze {
    rows: usize,
    columns: usize,
    layout: Vec<bool>, // false means wall, true means open
    guards: Vec<Guard>,
    robot_position: usize,
}

impl SingleMaze {
    fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let line_one = lines.next().unwrap();
        let mut line_one_split = line_one.split(' ').map(|n| n.parse::<usize>().unwrap());
        let rows = line_one_split.next().unwrap();
        let columns = line_one_split.next().unwrap();
        let mut layout = vec![];
        let mut initial_position = 0;
        for row in 0..rows {
            for (column, char_as_str) in lines.next().unwrap().split(' ').enumerate() {
                match char_as_str.chars().next().unwrap() {
                    '#' => layout.push(false),
                    '.' => layout.push(true),
                    'X' => {
                        initial_position = row * columns + column;
                        layout.push(true);
                    },
                    _ => panic!("Unexpected character"),
                }
            }
        }
        let number_guards = lines.next().unwrap().parse::<usize>().unwrap();
        let mut guards = vec![];
        for _ in 0..number_guards {
            let line_guard = lines.next().unwrap();
            let mut line_guard_split = line_guard.split(' ');
            let row = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let column = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let patrol_path_size = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let direction_str = line_guard_split.next().unwrap();
            guards.push(
                Guard::new(row * columns + column, patrol_path_size, Direction::from_char(direction_str.chars().next().unwrap()))
            )
        }
        SingleMaze {
            rows: rows,
            columns: columns,
            layout: layout,
            guards,
            robot_position: initial_position
        }
    }
}

pub struct Maze {
    maze_one: SingleMaze,
    maze_two: SingleMaze,
}

impl Maze {
    pub fn new<P>(path: P) -> Self
    where P: AsRef<Path> {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
        let maze_one = SingleMaze::from_lines(&mut lines);
        let maze_two = SingleMaze::from_lines(&mut lines);
        Maze { maze_one, maze_two}
    }

    pub fn write_solution<P>(&self, output_path: P) -> std::io::Result<()>
    where P: AsRef<Path> {
        let result = " ";
        let mut output_file = File::create(output_path)?;
        output_file.write_all(result.to_string().as_bytes())?;
        Ok(())
    }
}
