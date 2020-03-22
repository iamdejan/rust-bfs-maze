use std::io::{self, Write};
use std::vec::Vec;
use std::collections::{LinkedList, HashSet};
use std::string::String;
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug)]
struct Point {
    r: usize,
    c: usize
}

impl Point {
    fn new(r: usize, c: usize) -> Point {
        return Point {
            r: r,
            c: c
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.r == other.r && self.c == other.c;
    }
}

impl Hash for Point {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let data = [self.r as u8, self.c as u8];
        state.write(&data);
        state.finish();
    }
}

fn get_line() -> String {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");
    return buffer;
}

fn print_maze(maze: &Vec<Vec<char>>, number_of_rows: usize, number_of_columns: usize) {
    for r in 0..number_of_rows {
        for c in 0..number_of_columns {
            print!("{}", maze[r][c])
        }
        println!("")
    }
}

fn is_inside_maze(r: i64, c: i64, number_of_rows: i64, number_of_columns: i64) -> bool {
    return r >= 0 && r < number_of_rows && c >= 0 && c < number_of_columns;
}

fn bfs(maze: Vec<Vec<char>>, entry: Point, exit: Point) -> Result<u64, &'static str> {
    let neighbors_row: [i64; 4] = [-1, 0, 0, 1];
    let neighbors_column: [i64; 4] = [0, -1, 1, 0];
    let neighbors_size: usize = 4;

    let number_of_rows: i64 = maze.len() as i64;
    let number_of_columns: i64 = maze[0].len() as i64;

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: LinkedList<Point> = LinkedList::new();
    queue.push_back(entry);
    let mut moves: u64 = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let current: Point = queue.pop_front().unwrap();
            if current == exit {
                return Result::Ok(moves);
            }

            if maze[current.r][current.c] == '#' {
                continue;
            }

            if visited.contains(&current) {
                continue;
            }

            let current_row = current.r;
            let current_column = current.c;
            visited.insert(current);

            for i in 0..neighbors_size {
                let next_row = (current_row as i64) + neighbors_row[i];
                let next_column = (current_column as i64) + neighbors_column[i];

                if !is_inside_maze(next_row, next_column, number_of_rows, number_of_columns) {
                    continue;
                }

                let next_row = next_row as usize;
                let next_column = next_column as usize;
                if maze[next_row][next_column] == '#' {
                    continue;
                }

                let next_point: Point = Point::new(next_row, next_column);
                if visited.contains(&next_point) {
                    continue;
                }
                queue.push_back(next_point);
            }
        }
        moves += 1;
    }

    return Result::Err("Exit point is not found");
}

fn main() {
    let number_of_rows: usize = get_line().trim().parse().unwrap();
    let number_of_columns: usize = get_line().trim().parse().unwrap();

    let mut entry: Point = Point {
        r: 0,
        c: 0
    };
    let mut exit: Point = Point {
        r: 0,
        c: 0
    };

    let mut maze: Vec<Vec<char>> = Vec::new();
    for r in 0..number_of_rows {
        let row: Vec<char> = get_line().chars().collect();
        for c in 0..row.len() {
            if row[c] == 'X' {
                exit.r = r;
                exit.c = c;
            } else if row[c] == 'E' {
                entry.r = r;
                entry.c = c;
            }
        }
        maze.push(row);
    }

    println!("This is the maze:");
    print_maze(&maze, number_of_rows, number_of_columns);
    println!("");
    println!("Entry point: {:#?}", entry);
    println!("Exit point: {:#?}", exit);
    println!("");

    let result: Result<u64, &'static str> = bfs(maze, entry, exit);
    match result {
        Ok(v) => {
            println!("BFS result is {}", v);
        },
        Err(e) => {
            println!("Error caused by {:#?}", e);
        }
    }
    io::stdout().flush().unwrap();
}
