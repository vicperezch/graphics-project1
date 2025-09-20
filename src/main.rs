mod caster;
mod framebuffer;
mod maze;
mod player;

use framebuffer::Framebuffer;
use maze::{Maze, load_maze};
use player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

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

fn render3d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;

    let hw = framebuffer.width as f32 / 2.0; // precalculated half width
    let hh = framebuffer.height as f32 / 2.0; // precalculated half height

    framebuffer.set_current_color(Color::WHITESMOKE);

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // current ray divided by total rays
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        // Calculate the height of the stake
        let distance_to_wall = intersect.distance * 5.0; // how far is this wall from the player
        let distance_to_projection_plane = hw / (player.fov / 2.0).tan(); // how far is the "player" from the "camera"
        // this ratio doesn't really matter as long as it is a function of distance
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        // Calculate the position to draw the stake
        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Draw the stake directly in the framebuffer
        for y in stake_top..stake_bottom {
            framebuffer.set_pixel(i as i32, y as i32); // Assuming white color for the stake
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

    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();

        process_events(&window, &mut player);

        let mut mode = "3D";

        if window.is_key_down(KeyboardKey::KEY_M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        framebuffer.clear();

        if mode == "2D" {
            render_maze(&mut framebuffer, &maze, block_size);
            let num_rays = 100;
            for i in 0..num_rays {
                let current_ray = i as f32 / num_rays as f32;
                let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
                cast_ray(&mut framebuffer, &maze, &player, a, block_size, true);
            }
        } else {
            render3d(&mut framebuffer, &player);
        }

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(5));
    }
}
