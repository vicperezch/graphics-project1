use crate::enemy::Enemy;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> (Maze, Vec<Enemy>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maze = Vec::new();
    let mut enemies = Vec::new();
    let block_size = 100.0; // Same as in main

    for (row_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row = Vec::new();

        for (col_index, ch) in line.chars().enumerate() {
            if ch == 'e' || ch == 'E' {
                // Found an enemy, place it in the center of the cell
                let x = col_index as f32 * block_size + block_size / 2.0;
                let y = row_index as f32 * block_size + block_size / 2.0;
                enemies.push(Enemy::new(x, y));
                row.push(' '); // Replace with empty space in maze
            } else {
                row.push(ch);
            }
        }
        maze.push(row);
    }

    (maze, enemies)
}

