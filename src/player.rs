use crate::maze::Maze;
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(window: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 15.0;
    const COLLISION_MARGIN: f32 = 10.0; // Small margin to prevent getting too close to walls

    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(KeyboardKey::KEY_UP) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();

        // Check if new position would be inside a wall
        if is_valid_position(new_x, new_y, maze, block_size, COLLISION_MARGIN) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();

        // Check if new position would be inside a wall
        if is_valid_position(new_x, new_y, maze, block_size, COLLISION_MARGIN) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
}

fn is_valid_position(x: f32, y: f32, maze: &Maze, block_size: usize, margin: f32) -> bool {
    let positions = [
        (x - margin, y - margin),
        (x + margin, y - margin),
        (x - margin, y + margin),
        (x + margin, y + margin),
    ];

    for (px, py) in positions.iter() {
        // Convert to maze coordinates
        let i = (*px as usize) / block_size;
        let j = (*py as usize) / block_size;

        // Check bounds
        if j >= maze.len() || i >= maze[0].len() {
            return false;
        }

        // Check if position is in a wall
        if maze[j][i] != ' ' {
            return false;
        }
    }

    true
}
