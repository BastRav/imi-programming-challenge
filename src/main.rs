use imi_programming_challenge::maze::Maze;

fn main() {
    let maze = Maze::new("robots.in");
    let _ = maze.write_solution("robots.out");
}
