#[derive(Clone, PartialEq)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
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

    fn to_position_change(&self, columns: u8) -> i8 {
        match self {
            Self::North => -(columns as i8),
            Self::East => 1,
            Self::South => columns as i8,
            Self::West => -1,
        }
    }
}

#[derive(Clone)]
pub struct Guard {
    position: u16,
    patrol_path_size: u8,
    movement: i8,
    steps_to_starting_position: u8,
    reversed_direction: bool,
}

impl Guard {
    fn new(starting_position: u16, patrol_path_size: u8, movement: i8) -> Self {
        Self {
            position: starting_position,
            patrol_path_size,
            movement,
            steps_to_starting_position: 0,
            reversed_direction: false,
        }
    }

    fn step(&mut self) -> u16 {
        if self.steps_to_starting_position == 0 {
            self.reversed_direction = false;
        }
        else if self.steps_to_starting_position == self.patrol_path_size - 1 {
            self.reversed_direction = true;
        }
        let mut position_i16 = self.position as i16;
        if self.reversed_direction {
            self.steps_to_starting_position -= 1;
            position_i16 -= self.movement as i16;
        }
        else {
            self.steps_to_starting_position += 1;
            position_i16 += self.movement as i16;
        }
        self.position = position_i16 as u16;
        self.position
    }
}

#[derive(Clone)]
pub struct SingleMaze {
    columns: u8,
    layout: Vec<bool>, // false means wall, true means open
    guards: Vec<Guard>,
    robot_position: u16,
    pub robot_outside: bool,
    exits: Vec<(u16, Direction)>,
}

impl SingleMaze {
    pub fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let line_one = lines.next().unwrap();
        let mut line_one_split = line_one.split(' ').map(|n| n.parse::<u8>().unwrap());
        let rows = line_one_split.next().unwrap();
        let columns = line_one_split.next().unwrap();
        let mut layout = vec![];
        let mut initial_position = 0;
        let mut exits = vec![];
        for row in 0..rows {
            for (column, char) in (0..).zip(lines.next().unwrap().chars()) {
                match char {
                    '#' => layout.push(false),
                    '.' => {
                        layout.push(true);
                        let position = row as u16 * columns as u16 + column as u16;
                        if row == 0 {exits.push((position, Direction::North));}
                        if row == rows-1 {exits.push((position, Direction::South));}
                        if column == 0 {exits.push((position, Direction::West));}
                        if column == columns-1 {exits.push((position, Direction::East));}
                    },
                    'X' => {
                        let position = row as u16 * columns as u16 + column as u16;
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
        let number_guards = lines.next().unwrap().parse::<u8>().unwrap();
        let mut guards = vec![];
        for _ in 0..number_guards {
            let line_guard = lines.next().unwrap();
            let mut line_guard_split = line_guard.split(' ');
            let row = line_guard_split.next().unwrap().parse::<u16>().unwrap();
            let column = line_guard_split.next().unwrap().parse::<u16>().unwrap();
            let patrol_path_size = line_guard_split.next().unwrap().parse::<u8>().unwrap();
            let direction_str = line_guard_split.next().unwrap();
            let direction = Direction::from_char(direction_str.chars().next().unwrap());
            let position = row * columns as u16 + column;
            guards.push(
                Guard::new(position, patrol_path_size, direction.to_position_change(columns))
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

    pub fn get_hash(&self) -> u32 {
        let robot_outside_hash = if self.robot_outside {1} else {0}; // 1st bit
        let robot_position_hash = (self.robot_position as u32) << 1; // max 400 -> 2nd to 10th
        return robot_outside_hash + robot_position_hash;
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
        let new_robot_position = (self.robot_position as i16 + robot_move as i16) as u16;
        if self.layout[new_robot_position as usize] {
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
