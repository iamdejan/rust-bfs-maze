use std::io::{self, Write};
use std::vec::Vec;
use std::string::String;

#[derive(Debug)]
struct Point {
    r: usize,
    c: usize
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

fn main() {
    let number_of_rows: usize = get_line().trim().parse().unwrap();
    let number_of_columns: usize = get_line().trim().parse().unwrap();

    let mut entry: Point = Point{
        r: 0,
        c: 0
    };
    let mut exit: Point = Point{
        r: 0,
        c: 0
    };

    let mut maze: Vec<Vec<char>> = Vec::new();
    for r in 0..number_of_rows {
        let row: Vec<char> = get_line().chars().collect();
        for c in 0..row.len() {
            if row[c] == 'X' {
                exit = Point{
                    r: r,
                    c: c
                };
            } else if row[c] == 'E' {
                entry = Point {
                    r: r,
                    c: c
                }
            }
        }
        maze.push(row);
        
    }
    
    print_maze(&maze, number_of_rows, number_of_columns);
    println!("Entry point: {:#?}", entry);
    println!("Exit point: {:#?}", exit);
    io::stdout().flush().unwrap();
}
