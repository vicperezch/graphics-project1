mod caster;
mod maze;
mod player;
mod wall_textures;

use maze::{Maze, load_maze};
use player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;
use wall_textures::WallTextures;

use crate::caster::cast_ray;
use crate::player::process_events;

fn render3d(
    d: &mut RaylibDrawHandle,
    player: &Player,
    maze: &Maze,
    block_size: usize,
    wall_textures: &WallTextures,
    window_width: i32,
    window_height: i32,
) {
    let num_rays = 320;
    let width = window_width as f32;
    let height = window_height as f32;

    let hw = width / 2.0;
    let hh = height / 2.0;

    // Draw sky and floor
    d.draw_rectangle(
        0,
        0,
        window_width,
        window_height / 2,
        Color::new(135, 206, 235, 255),
    );
    d.draw_rectangle(
        0,
        window_height / 2,
        window_width,
        window_height / 2,
        Color::BLACK,
    );

    // Pre-calculate constants
    let distance_to_projection_plane = hw / (player.fov / 2.0).tan();
    let fov_start = player.a - (player.fov / 2.0);
    let fov_step = player.fov / num_rays as f32;
    let column_width = (width / num_rays as f32).ceil() as i32;

    for i in 0..num_rays {
        let a = fov_start + (i as f32 * fov_step);

        let intersect = cast_ray(&maze, &player, a, block_size);

        if intersect.distance > 4000.0 {
            continue;
        }

        let corrected_distance = intersect.perpendicular_distance.max(10.0);
        let wall_height = ((block_size as f32 * distance_to_projection_plane) / corrected_distance)
            .min(height * 2.0);

        // Calculate the actual texture portion that's visible
        let wall_top_unclamped = hh - wall_height / 2.0;
        let wall_bottom_unclamped = hh + wall_height / 2.0;

        // Screen boundaries
        let wall_top = wall_top_unclamped.max(0.0) as i32;
        let wall_bottom = wall_bottom_unclamped.min(height) as i32;

        // Calculate texture offset for walls that extend beyond screen
        let tex_start = if wall_top_unclamped < 0.0 {
            // Wall extends above screen - start texture partway through
            ((-wall_top_unclamped / wall_height) * 128.0) as usize
        } else {
            0
        };

        let tex_end = if wall_bottom_unclamped > height {
            // Wall extends below screen - end texture partway through
            (((height - wall_top_unclamped) / wall_height) * 128.0) as usize
        } else {
            128
        };

        let x = (i as f32 * column_width as f32) as i32;

        if wall_textures.is_enabled() {
            // Adaptive strip height based on wall height
            let strip_height = if corrected_distance < 50.0 {
                16 // Very close
            } else if corrected_distance < 100.0 {
                8 // Close
            } else {
                4 // Normal
            };

            let visible_height = wall_bottom - wall_top;
            let tex_range = tex_end - tex_start;
            let tex_step = tex_range as f32 / visible_height as f32;

            // Limit strips for performance
            let max_strips = 50;
            let actual_strip_height = (visible_height / max_strips).max(strip_height);

            let mut current_tex_y = tex_start as f32;

            for y in (wall_top..wall_bottom).step_by(actual_strip_height as usize) {
                let strip_end = (y + actual_strip_height).min(wall_bottom);
                let tex_y = current_tex_y as usize;

                let color = wall_textures.get_pixel(intersect.tx, tex_y.min(127), intersect.impact);

                let shade = (1.0 - (corrected_distance / 1500.0)).max(0.3).min(1.0);
                let shaded_color = Color::new(
                    (color.r as f32 * shade) as u8,
                    (color.g as f32 * shade) as u8,
                    (color.b as f32 * shade) as u8,
                    255,
                );

                d.draw_rectangle(x, y, column_width + 1, (strip_end - y), shaded_color);

                current_tex_y += (strip_end - y) as f32 * tex_step;
            }
        } else {
            let base_color = match intersect.impact {
                '+' | '-' => Color::DARKGRAY,
                '|' => Color::GRAY,
                _ => Color::LIGHTGRAY,
            };

            let shade = (1.0 - (corrected_distance / 1500.0)).max(0.3).min(1.0);
            let color = Color::new(
                (base_color.r as f32 * shade) as u8,
                (base_color.g as f32 * shade) as u8,
                (base_color.b as f32 * shade) as u8,
                255,
            );

            d.draw_rectangle(
                x,
                wall_top,
                column_width + 1,
                (wall_bottom - wall_top),
                color,
            );
        }
    }
}

fn render_minimap(
    d: &mut RaylibDrawHandle,
    maze: &Maze,
    player: &Player,
    window_width: i32,
    block_size: usize,
) {
    let minimap_scale = 8i32; // Change to i32
    let minimap_block_size = (block_size as i32 / minimap_scale);
    let minimap_width = (maze[0].len() as i32) * minimap_block_size;
    let minimap_height = (maze.len() as i32) * minimap_block_size;
    let margin = 20;

    let minimap_x = window_width - minimap_width - margin;
    let minimap_y = margin;

    // Draw minimap background
    d.draw_rectangle(
        minimap_x - 2,
        minimap_y - 2,
        minimap_width + 4,
        minimap_height + 4,
        Color::new(0, 0, 0, 180),
    );

    // Draw maze walls
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if cell != ' ' {
                let x = minimap_x + (col_index as i32 * minimap_block_size);
                let y = minimap_y + (row_index as i32 * minimap_block_size);
                d.draw_rectangle(x, y, minimap_block_size, minimap_block_size, Color::GRAY);
            }
        }
    }

    // Draw player
    let player_x = minimap_x + ((player.pos.x as i32) / minimap_scale);
    let player_y = minimap_y + ((player.pos.y as i32) / minimap_scale);
    d.draw_circle(player_x, player_y, 3.0, Color::GREEN);

    // Draw direction
    let dir_x = player_x + (15.0 * player.a.cos()) as i32;
    let dir_y = player_y + (15.0 * player.a.sin()) as i32;
    d.draw_line(player_x, player_y, dir_x, dir_y, Color::RED);
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

    window.disable_cursor();
    window.set_target_fps(60);

    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    let maze = load_maze("maze.txt");
    let wall_textures = WallTextures::new();

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            if window.is_cursor_hidden() {
                window.enable_cursor();
            } else {
                window.disable_cursor();
            }
        }

        process_events(&window, &mut player, &maze, block_size);

        // Get FPS before mutable borrow
        let fps = window.get_fps();

        // All rendering in one draw call
        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::BLACK);

        // Render everything using GPU-accelerated draw calls
        render3d(
            &mut d,
            &player,
            &maze,
            block_size,
            &wall_textures,
            window_width,
            window_height,
        );
        render_minimap(&mut d, &maze, &player, window_width, block_size);

        // FPS counter
        d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::GREEN);
    }
}

