use imi_programming_challenge::maze::Maze;

fn main() {
    let mut maze = Maze::new("robots.in");
    let _ = maze.write_solution("robots.out");
}
