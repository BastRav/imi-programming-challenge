#[derive(Clone, PartialEq, Hash)]
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

#[derive(Clone, Hash)]
pub struct Guard {
    position: usize,
    patrol_path_size: usize,
    movement: isize,
    steps_to_starting_position: usize,
    reversed_direction: bool,
}

impl Guard {
    fn new(starting_position: usize, patrol_path_size: usize, movement: isize) -> Self {
        Self {
            position: starting_position,
            patrol_path_size,
            movement,
            steps_to_starting_position: 0,
            reversed_direction: false,
        }
    }

    fn step(&mut self) -> usize {
        if self.steps_to_starting_position == 0 {
            self.reversed_direction = false;
        }
        else if self.steps_to_starting_position == self.patrol_path_size - 1 {
            self.reversed_direction = true;
        }
        let mut position_isize = self.position as isize;
        if self.reversed_direction {
            self.steps_to_starting_position -= 1;
            position_isize -= self.movement;
        }
        else {
            self.steps_to_starting_position += 1;
            position_isize += self.movement;
        }
        self.position = position_isize as usize;
        self.position
    }
}

#[derive(Clone, Hash)]
pub struct SingleMaze {
    columns: usize,
    layout: Vec<bool>, // false means wall, true means open
    guards: Vec<Guard>,
    robot_position: usize,
    pub robot_outside: bool,
    exits: Vec<(usize, Direction)>,
}

impl SingleMaze {
    pub fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
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
        for _ in 0..number_guards {
            let line_guard = lines.next().unwrap();
            let mut line_guard_split = line_guard.split(' ');
            let row = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let column = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let patrol_path_size = line_guard_split.next().unwrap().parse::<usize>().unwrap();
            let direction_str = line_guard_split.next().unwrap();
            let direction = Direction::from_char(direction_str.chars().next().unwrap());
            guards.push(
                Guard::new(row * columns + column, patrol_path_size, direction.to_position_change(columns))
            );
        }
        SingleMaze {
            columns: columns,
            layout: layout,
            guards,
            robot_position: initial_position,
            robot_outside: false,
            exits,
        }
    }

    pub fn step(&mut self, direction: &Direction) -> bool {
        if self.robot_outside {
            // already won
            return true;
        }
        for (position_exit, direction_exit) in self.exits.iter() {
            if self.robot_position == *position_exit && direction == direction_exit {
                // it's a win
                self.robot_outside = true;
                return true;
            }
        }
        let robot_move = direction.to_position_change(self.columns);
        let new_robot_position = (self.robot_position as isize + robot_move) as usize;
        if self.layout[new_robot_position] {
            for guard in self.guards.iter_mut(){
                if guard.position == new_robot_position || guard.step() == new_robot_position {
                    // caught by a guard
                    return false;
                }
            }
            // move is possible
            self.robot_position = new_robot_position;
            return true;
        }
        else {
            // hit a wall, it's allowed but you do not move
            return true;
        }
    }
}
