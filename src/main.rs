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

fn render_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    original_block_size: usize,
) {
    // Minimap configuration
    let minimap_scale = 5; // How much smaller the minimap blocks are
    let minimap_block_size = original_block_size / minimap_scale;
    let minimap_width = maze[0].len() * minimap_block_size;
    let minimap_height = maze.len() * minimap_block_size;
    let margin = 20; // Margin from screen edges
    let padding = 10; // Padding inside minimap border

    // Position minimap in top-right corner
    let minimap_x = framebuffer.width as usize - minimap_width - margin - padding * 2;
    let minimap_y = margin;

    // Draw minimap background (dark transparent effect)
    framebuffer.set_current_color(Color::new(0, 0, 0, 180));
    for x in (minimap_x - padding)..(minimap_x + minimap_width + padding) {
        for y in (minimap_y - padding)..(minimap_y + minimap_height + padding) {
            framebuffer.set_pixel(x as i32, y as i32);
        }
    }

    // Draw minimap border
    framebuffer.set_current_color(Color::new(100, 100, 100, 255));
    // Top and bottom borders
    for x in (minimap_x - padding)..(minimap_x + minimap_width + padding) {
        framebuffer.set_pixel(x as i32, (minimap_y - padding) as i32);
        framebuffer.set_pixel(x as i32, (minimap_y + minimap_height + padding - 1) as i32);
    }
    // Left and right borders
    for y in (minimap_y - padding)..(minimap_y + minimap_height + padding) {
        framebuffer.set_pixel((minimap_x - padding) as i32, y as i32);
        framebuffer.set_pixel((minimap_x + minimap_width + padding - 1) as i32, y as i32);
    }

    // Draw maze walls
    framebuffer.set_current_color(Color::new(150, 150, 150, 255));
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell != ' ' {
                let x_start = minimap_x + col_index * minimap_block_size;
                let y_start = minimap_y + row_index * minimap_block_size;

                for x in x_start..(x_start + minimap_block_size) {
                    for y in y_start..(y_start + minimap_block_size) {
                        framebuffer.set_pixel(x as i32, y as i32);
                    }
                }
            }
        }
    }

    // Calculate player position on minimap
    let player_minimap_x = minimap_x as f32 + (player.pos.x / minimap_scale as f32);
    let player_minimap_y = minimap_y as f32 + (player.pos.y / minimap_scale as f32);

    // Draw player position
    framebuffer.set_current_color(Color::new(0, 255, 0, 255)); // Green
    let player_size = 3; // Size of player dot
    for x in -player_size..=player_size {
        for y in -player_size..=player_size {
            if x * x + y * y <= player_size * player_size {
                // Circle shape
                framebuffer.set_pixel((player_minimap_x as i32) + x, (player_minimap_y as i32) + y);
            }
        }
    }

    // Draw player direction indicator
    framebuffer.set_current_color(Color::new(255, 0, 0, 255)); // Red
    for i in 0..15 {
        let x = player_minimap_x + (i as f32 * player.a.cos());
        let y = player_minimap_y + (i as f32 * player.a.sin());
        framebuffer.set_pixel(x as i32, y as i32);
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

    // Lock and hide the mouse cursor for FPS-style controls
    window.disable_cursor();

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

        // Handle mouse lock/unlock with ESC key
        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            if window.is_cursor_hidden() {
                window.enable_cursor();
            } else {
                window.disable_cursor();
            }
        }

        process_events(&window, &mut player, &maze, block_size);

        // Always render 3D view
        render3d(&mut framebuffer, &player, &maze, block_size);

        // Always render minimap on top
        render_minimap(&mut framebuffer, &maze, &player, block_size);

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}

