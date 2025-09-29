use crate::enemy::Enemy;
use raylib::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> (Maze, Vec<Enemy>, Option<Vector2>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maze = Vec::new();
    let mut enemies = Vec::new();
    let mut finish_pos = None;
    let block_size = 100.0;

    for (row_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row = Vec::new();

        for (col_index, ch) in line.chars().enumerate() {
            if ch == 'e' || ch == 'E' {
                // Found an enemy, place it in the center of the cell
                let x = col_index as f32 * block_size + block_size / 2.0;
                let y = row_index as f32 * block_size + block_size / 2.0;
                enemies.push(Enemy::new(x, y));
                row.push(' ');
            } else if ch == 'w' || ch == 'W' {
                // Found the finish/win position, place it in the center of the cell
                let x = col_index as f32 * block_size + block_size / 2.0;
                let y = row_index as f32 * block_size + block_size / 2.0;
                finish_pos = Some(Vector2::new(x, y));
                row.push(' ');
            } else {
                row.push(ch);
            }
        }
        maze.push(row);
    }

    (maze, enemies, finish_pos)
}
