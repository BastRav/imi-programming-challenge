use imi_programming_challenge::maze::Maze;

fn main() {
    let (maze, maze_state) = Maze::new("robots.in");
    let _ = maze.write_solution(maze_state, "robots.out");
}
