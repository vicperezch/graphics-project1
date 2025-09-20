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
    const MOUSE_SENSITIVITY: f32 = 0.003; // Mouse sensitivity for horizontal rotation
    const COLLISION_MARGIN: f32 = 10.0; // Small margin to prevent getting too close to walls

    // Mouse control for horizontal camera rotation
    let mouse_delta = window.get_mouse_delta();
    player.a += mouse_delta.x * MOUSE_SENSITIVITY;

    // Keep angle in valid range
    if player.a > 2.0 * PI {
        player.a -= 2.0 * PI;
    } else if player.a < 0.0 {
        player.a += 2.0 * PI;
    }

    // WASD movement
    if window.is_key_down(KeyboardKey::KEY_W) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();

        // Check if new position would be inside a wall
        if is_valid_position(new_x, new_y, maze, block_size, COLLISION_MARGIN) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();

        // Check if new position would be inside a wall
        if is_valid_position(new_x, new_y, maze, block_size, COLLISION_MARGIN) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        // Strafe left (perpendicular to viewing direction)
        let strafe_angle = player.a - PI / 2.0;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();

        if is_valid_position(new_x, new_y, maze, block_size, COLLISION_MARGIN) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        // Strafe right (perpendicular to viewing direction)
        let strafe_angle = player.a + PI / 2.0;
        let new_x = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * strafe_angle.sin();

        if is_valid_position(new_x, new_y, maze, block_size, COLLISION_MARGIN) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
}

fn is_valid_position(x: f32, y: f32, maze: &Maze, block_size: usize, margin: f32) -> bool {
    // Check all four corners of the player's bounding box
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
