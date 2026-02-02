use std::hash::{Hash, Hasher};

use strum_macros::EnumIter;

#[derive(EnumIter, Clone, PartialEq)]
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

#[derive(Clone)]
struct GuardState {
    position: usize,
    reversed_direction: bool,
    steps_to_starting_position: usize,
}

impl Hash for GuardState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reversed_direction.hash(state);
        self.steps_to_starting_position.hash(state);
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

#[derive(Clone, Hash)]
pub struct SingleMazeState {
    guards_states: Vec<GuardState>,
    robot_position: usize,
    pub robot_outside: bool,
}

pub struct SingleMaze {
    columns: usize,
    layout: Vec<bool>, // false means wall, true means open
    guards: Vec<Guard>,
    exits: Vec<(usize, Direction)>,
}

impl SingleMaze {
    pub fn from_lines(lines: &mut impl Iterator<Item = String>) -> (Self, SingleMazeState) {
        let line_one = lines.next().unwrap();
        let mut line_one_split = line_one.split(' ').map(|n| n.parse::<usize>().unwrap());
        let rows = line_one_split.next().unwrap();
        let columns = line_one_split.next().unwrap();
        let mut layout = Vec::with_capacity(rows*columns);
        let mut initial_position = 0;
        let mut exits = Vec::with_capacity(2*(rows+columns));
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
        let mut guards = Vec::with_capacity(10);
        let mut guards_states = Vec::with_capacity(10);
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
        let state = SingleMazeState {
            robot_position: initial_position,
            robot_outside: false,
            guards_states: guards_states,
        };
        let maze = SingleMaze {
            columns: columns,
            layout: layout,
            guards,
            exits,
        };
        (maze, state)
    }

    pub fn no_exit(&self) -> bool {
        self.exits.len() == 0
    }

    pub fn step(&self, state: &SingleMazeState, direction: &Direction) -> (bool, SingleMazeState) {
        let mut new_state = state.clone();
        if state.robot_outside {
            // already won
            return (true, new_state);
        }
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
