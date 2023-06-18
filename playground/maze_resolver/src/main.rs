use std::{
    collections,
    fmt::Error,
    fs::{self, File},
    io::{self, BufRead},
};

fn main() {
    let mut maze = Maze::new();
    maze.print();

    maze.read_from_file(String::from("./config/maze.in"));
    maze.print();

    let result = maze.solve(Point::new(0, 0), Point::new(maze.row, maze.col));
    println!("maze step:");
    for row in result {
        for val in row {
            print!("{:>3}", val)
        }
        println!()
    }
}

const NOT_EXPLORED: i32 = 0;
const WALL: i32 = 1;

struct Maze {
    row: i32,
    col: i32,
    map: Vec<Vec<i32>>,
}

impl Maze {
    fn new() -> Self {
        Maze {
            row: 0,
            col: 0,
            map: Vec::new(),
        }
    }

    fn read_from_file(&mut self, path: String) {
        let mut lines = read_lines(path);
        if let Some(line) = lines.next() {
            if let Ok(line) = line {
                self.set_row_col(line)
            }
        }
        for line in lines {
            if let Ok(line) = line {
                self.set_map(line)
            }
        }
    }

    fn set_row_col(&mut self, line: String) {
        if let Ok(row) = line.split(" ").collect::<Vec<&str>>()[0].parse() {
            self.row = row
        }
        if let Ok(col) = line.split(" ").collect::<Vec<&str>>()[1].parse() {
            self.col = col
        }
    }

    fn set_map(&mut self, line: String) {
        self.map
            .push(line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
    }

    fn print(&self) {
        println!("row, col:, {}, {}", self.row, self.col);
        println!("maze:");
        for row in &self.map {
            for val in row {
                print!("{:>3}", val)
            }
            println!()
        }
    }

    fn solve(&self, start: Point, end: Point) -> Vec<Vec<i32>> {
        let dirs = Point::bfs_dirs();

        let mut steps = vec![vec![0; self.col as usize]; self.row as usize];
        let mut q = collections::VecDeque::from(vec![start]);

        while let Some(cur) = q.pop_front() {
            if cur == end {
                break;
            }

            for dir in &dirs {
                let next = cur.add(&dir);

                if let Ok(val) = next.at(&self.map) {
                    if val == WALL {
                        continue;
                    }
                } else {
                    continue;
                }

                if let Ok(val) = next.at(&steps) {
                    if val != NOT_EXPLORED {
                        continue;
                    }
                }

                if next == start {
                    continue;
                }

                if let Ok(val) = cur.at(&steps) {
                    steps[next.i as usize][next.j as usize] = val + 1;
                }

                q.push_back(next);
            }
        }

        steps
    }
}

fn read_lines(filename: String) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

#[derive(Clone, Debug, PartialEq, Copy)]
struct Point {
    i: i32,
    j: i32,
}

impl Point {
    fn new(i: i32, j: i32) -> Point {
        Point { i, j }
    }

    fn bfs_dirs() -> Vec<Point> {
        vec![
            Point::new(-1, 0),
            Point::new(0, -1),
            Point::new(1, 0),
            Point::new(0, 1),
        ]
    }

    fn add(&self, p: &Point) -> Point {
        Point {
            i: self.i + p.i,
            j: self.j + p.j,
        }
    }

    fn at(&self, grid: &Vec<Vec<i32>>) -> Result<i32, ()> {
        if self.i < 0 || self.i >= grid.len() as i32 {
            return Err(());
        }

        if self.j < 0 || self.j >= grid[0].len() as i32 {
            return Err(());
        }

        Ok(grid[self.i as usize][self.j as usize])
    }
}
