mod caster;
mod framebuffer;
mod maze;
mod player;

use framebuffer::Framebuffer;
use maze::{Maze, load_maze};
use player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;

use crate::caster::cast_ray;
use crate::player::process_events;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(Color::RED);

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }
}

pub fn render_maze(framebuffer: &mut Framebuffer, maze: &Maze, block_size: usize) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = col_index * block_size;
            let yo = row_index * block_size;

            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Maze, block_size: usize) {
    let num_rays = framebuffer.width / 2; // Reduce ray count for performance
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    let hw = width / 2.0;
    let hh = height / 2.0;

    framebuffer.set_current_color(Color::WHITESMOKE);

    // Pre-calculate constants
    let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
    let fov_start = player.a - (player.fov / 2.0);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = fov_start + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Skip rendering if ray didn't hit anything
        if intersect.distance > 4000.0 {
            continue;
        }

        // Apply fisheye correction
        let corrected_distance = intersect.distance * ((player.a - a).cos());

        // Calculate the height of the wall slice
        let stake_height = (block_size as f32 * distance_to_projection_plane) / corrected_distance;

        // Calculate vertical position
        let stake_top = ((hh - stake_height / 2.0).max(0.0)) as i32;
        let stake_bottom = ((hh + stake_height / 2.0).min(height - 1.0)) as i32;

        // Draw two pixels wide for better coverage since we're using half resolution
        let x = i * 2;
        for y in stake_top..stake_bottom {
            framebuffer.set_pixel(x as i32, y);
            framebuffer.set_pixel((x + 1) as i32, y);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as i32, window_height as i32, Color::BLACK);
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    // Load the maze once before the loop
    let maze = load_maze("maze.txt");

    let mut mode = "3D";

    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();

        process_events(&window, &mut player, &maze, block_size);

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        framebuffer.clear();

        if mode == "2D" {
            render_maze(&mut framebuffer, &maze, block_size);
            let num_rays = 50; // Reduced from 100
            for i in 0..num_rays {
                let current_ray = i as f32 / num_rays as f32;
                let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
                cast_ray(&mut framebuffer, &maze, &player, a, block_size, true);
            }
        } else {
            render3d(&mut framebuffer, &player, &maze, block_size);
        }

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}

