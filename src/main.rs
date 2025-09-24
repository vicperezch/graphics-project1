mod caster;
mod enemy;
mod maze;
mod player;
mod wall_textures;

use enemy::Enemy;
use maze::{Maze, load_maze};
use player::Player;
use raylib::prelude::*;
use std::f32::consts::PI;
use wall_textures::WallTextures;

use crate::caster::cast_ray;
use crate::player::process_events;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
}

struct MenuOption {
    text: String,
    action: fn() -> GameState,
}

fn start_game() -> GameState {
    GameState::Playing
}

fn quit_game() -> GameState {
    std::process::exit(0);
}

fn render_menu(
    d: &mut RaylibDrawHandle,
    window_width: i32,
    window_height: i32,
    selected_option: usize,
) {
    // Draw background
    d.clear_background(Color::new(30, 30, 40, 255));

    // Title
    let title = "Raycaster Game";
    let title_font_size = 60;
    let title_width = d.measure_text(title, title_font_size);
    let title_x = (window_width - title_width) / 2;
    let title_y = window_height / 4;

    d.draw_text(title, title_x, title_y, title_font_size, Color::WHITE);

    // Menu options
    let options = vec![
        MenuOption {
            text: "Start".to_string(),
            action: start_game,
        },
        MenuOption {
            text: "Quit".to_string(),
            action: quit_game,
        },
    ];

    let option_font_size = 40;
    let option_spacing = 60;
    let options_start_y = window_height / 2;

    for (i, option) in options.iter().enumerate() {
        let option_width = d.measure_text(&option.text, option_font_size);
        let option_x = (window_width - option_width) / 2;
        let option_y = options_start_y + (i as i32 * option_spacing);

        let color = if i == selected_option {
            Color::YELLOW
        } else {
            Color::LIGHTGRAY
        };

        d.draw_text(&option.text, option_x, option_y, option_font_size, color);

        // Draw selection indicator
        if i == selected_option {
            let arrow_x = option_x - 40;
            d.draw_text(">", arrow_x, option_y, option_font_size, Color::YELLOW);
        }
    }

    // Instructions
    let instructions = "Use UP/DOWN arrows to select, ENTER to confirm";
    let inst_font_size = 20;
    let inst_width = d.measure_text(instructions, inst_font_size);
    let inst_x = (window_width - inst_width) / 2;
    let inst_y = window_height - 100;

    d.draw_text(instructions, inst_x, inst_y, inst_font_size, Color::GRAY);
}

fn render3d(
    d: &mut RaylibDrawHandle,
    player: &Player,
    maze: &Maze,
    block_size: usize,
    wall_textures: &WallTextures,
    window_width: i32,
    window_height: i32,
    zbuffer: &mut Vec<f32>,
) {
    let num_rays = 320;
    let width = window_width as f32;
    let height = window_height as f32;

    let hw = width / 2.0;
    let hh = height / 2.0;

    // Clear zbuffer
    zbuffer.clear();
    zbuffer.resize(window_width as usize, f32::MAX);

    // Draw sky and floor
    d.draw_rectangle(
        0,
        0,
        window_width,
        window_height / 2,
        Color::new(25, 25, 35, 255), // Dark blue-gray sky
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

        // Store depth in zbuffer for all columns this ray covers
        let x_start = (i as f32 * column_width as f32) as i32;
        let x_end = ((i + 1) as f32 * column_width as f32) as i32;
        for x in x_start..x_end {
            if x >= 0 && x < window_width {
                zbuffer[x as usize] = corrected_distance;
            }
        }

        let wall_height = ((block_size as f32 * distance_to_projection_plane) / corrected_distance)
            .min(height * 2.0);

        let wall_top_unclamped = hh - wall_height / 2.0;
        let wall_bottom_unclamped = hh + wall_height / 2.0;

        let wall_top = wall_top_unclamped.max(0.0) as i32;
        let wall_bottom = wall_bottom_unclamped.min(height) as i32;

        let tex_start = if wall_top_unclamped < 0.0 {
            ((-wall_top_unclamped / wall_height) * 128.0) as usize
        } else {
            0
        };

        let tex_end = if wall_bottom_unclamped > height {
            (((height - wall_top_unclamped) / wall_height) * 128.0) as usize
        } else {
            128
        };

        let x = (i as f32 * column_width as f32) as i32;

        if wall_textures.is_enabled() {
            let strip_height = if corrected_distance < 50.0 {
                16
            } else if corrected_distance < 100.0 {
                8
            } else {
                4
            };

            let visible_height = wall_bottom - wall_top;
            let tex_range = tex_end - tex_start;
            let tex_step = tex_range as f32 / visible_height as f32;

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

                d.draw_rectangle(x, y, column_width + 1, strip_end - y, shaded_color);

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

            d.draw_rectangle(x, wall_top, column_width + 1, wall_bottom - wall_top, color);
        }
    }
}

fn render_enemies(
    d: &mut RaylibDrawHandle,
    player: &Player,
    enemies: &[Enemy],
    wall_textures: &WallTextures,
    window_width: i32,
    window_height: i32,
    zbuffer: &[f32],
) {
    if !wall_textures.is_enemy_enabled() {
        return;
    }

    let hw = window_width as f32 / 2.0;
    let hh = window_height as f32 / 2.0;
    let distance_to_projection_plane = hw / (player.fov / 2.0).tan();

    // Sort enemies by distance (furthest first)
    let mut sorted_enemies: Vec<(usize, f32)> = enemies
        .iter()
        .enumerate()
        .map(|(i, enemy)| {
            let dx = enemy.pos.x - player.pos.x;
            let dy = enemy.pos.y - player.pos.y;
            let dist = (dx * dx + dy * dy).sqrt();
            (i, dist)
        })
        .collect();
    sorted_enemies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (enemy_idx, distance) in sorted_enemies {
        let enemy = &enemies[enemy_idx];

        let dx = enemy.pos.x - player.pos.x;
        let dy = enemy.pos.y - player.pos.y;
        let sprite_angle = dy.atan2(dx);

        let mut angle_diff = sprite_angle - player.a;
        while angle_diff > PI {
            angle_diff -= 2.0 * PI;
        }
        while angle_diff < -PI {
            angle_diff += 2.0 * PI;
        }

        if angle_diff.abs() > player.fov / 2.0 + 0.2 {
            continue;
        }

        if distance < 20.0 || distance > 1500.0 {
            continue;
        }

        let sprite_height = (100.0 * distance_to_projection_plane) / distance;
        let sprite_width = sprite_height;

        let screen_x = hw + (angle_diff.tan() * distance_to_projection_plane);
        let x_start = (screen_x - sprite_width / 2.0) as i32;
        let x_end = (screen_x + sprite_width / 2.0) as i32;
        let y_start = (hh - sprite_height / 2.0) as i32;
        let y_end = (hh + sprite_height / 2.0) as i32;

        if x_end < 0 || x_start >= window_width || y_end < 0 || y_start >= window_height {
            continue;
        }

        let clipped_x_start = x_start.max(0);
        let clipped_x_end = x_end.min(window_width);
        let clipped_y_start = y_start.max(0);
        let clipped_y_end = y_end.min(window_height);

        // Check if enemy center is behind a wall
        let center_x = screen_x as i32;
        if center_x >= 0 && center_x < window_width {
            if distance >= zbuffer[center_x as usize] {
                continue;
            }
        }

        // Dynamic strip width based on sprite size to maintain performance
        let sprite_screen_width = clipped_x_end - clipped_x_start;
        let strip_width = if sprite_screen_width > 300 {
            16 // Very large sprite
        } else if sprite_screen_width > 150 {
            8 // Large sprite
        } else if sprite_screen_width > 75 {
            4 // Medium sprite
        } else {
            2 // Small sprite - keep detail
        };

        // Dynamic vertical strip height for large sprites
        let sprite_screen_height = clipped_y_end - clipped_y_start;
        let y_strip = if sprite_screen_height > 400 {
            12 // Very tall sprite
        } else if sprite_screen_height > 200 {
            8 // Tall sprite
        } else {
            4 // Normal height
        };

        // Limit total strips for performance
        let max_x_strips = 30;
        let actual_strip_width = (sprite_screen_width / max_x_strips).max(strip_width);

        for x in (clipped_x_start..clipped_x_end).step_by(actual_strip_width as usize) {
            // Check zbuffer for this column
            if x >= 0 && x < window_width && distance >= zbuffer[x as usize] {
                continue;
            }

            let strip_end = (x + actual_strip_width).min(clipped_x_end);
            let tex_x = (((x - x_start) as f32 / sprite_width * 128.0) as usize).min(127);

            for y in (clipped_y_start..clipped_y_end).step_by(y_strip) {
                let strip_height = (y + y_strip as i32).min(clipped_y_end) - y;
                let tex_y = (((y - y_start) as f32 / sprite_height * 128.0) as usize).min(127);

                let color = wall_textures.get_pixel(tex_x, tex_y, 'e');

                if color.a < 10 {
                    continue;
                }

                let shade = (1.0 - (distance / 800.0)).max(0.4).min(1.0);
                let shaded_color = Color::new(
                    (color.r as f32 * shade) as u8,
                    (color.g as f32 * shade) as u8,
                    (color.b as f32 * shade) as u8,
                    color.a,
                );

                d.draw_rectangle(x, y, strip_end - x, strip_height, shaded_color);
            }
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
    let minimap_scale = 8i32;
    let minimap_block_size = block_size as i32 / minimap_scale;
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

    window.set_target_fps(60);

    // Disable ESC as exit key
    window.set_exit_key(None);

    // Game state
    let mut game_state = GameState::Menu;
    let mut selected_option = 0;
    let num_options = 2;

    // Game resources
    let (maze, enemies) = load_maze("maze.txt");
    let wall_textures = WallTextures::new();
    let mut zbuffer: Vec<f32> = vec![f32::MAX; window_width as usize];

    // Player - will be reset each time game starts
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    println!("Loaded {} enemies from maze", enemies.len());

    while !window.window_should_close() {
        match game_state {
            GameState::Menu => {
                // Handle menu input
                if window.is_key_pressed(KeyboardKey::KEY_UP) {
                    if selected_option > 0 {
                        selected_option -= 1;
                    }
                }
                if window.is_key_pressed(KeyboardKey::KEY_DOWN) {
                    if selected_option < num_options - 1 {
                        selected_option += 1;
                    }
                }
                if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    // Execute the selected option
                    if selected_option == 0 {
                        // Reset player to initial state when starting game
                        player = Player {
                            pos: Vector2::new(150.0, 150.0),
                            a: PI / 3.0,
                            fov: PI / 3.0,
                        };
                        game_state = GameState::Playing;
                        window.disable_cursor();
                    } else if selected_option == 1 {
                        break; // Exit the game loop
                    }
                }

                // Render menu
                let mut d = window.begin_drawing(&raylib_thread);
                render_menu(&mut d, window_width, window_height, selected_option);
            }

            GameState::Playing => {
                // Check for ESC key BEFORE processing other events
                if window.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    // Return to menu
                    game_state = GameState::Menu;
                    window.enable_cursor();
                    selected_option = 0;
                    continue; // Skip to next iteration of the game loop
                }

                // Process game events
                process_events(&window, &mut player, &maze, block_size);

                // Get FPS before mutable borrow
                let fps = window.get_fps();

                // Render game
                let mut d = window.begin_drawing(&raylib_thread);
                d.clear_background(Color::BLACK);

                render3d(
                    &mut d,
                    &player,
                    &maze,
                    block_size,
                    &wall_textures,
                    window_width,
                    window_height,
                    &mut zbuffer,
                );
                render_enemies(
                    &mut d,
                    &player,
                    &enemies,
                    &wall_textures,
                    window_width,
                    window_height,
                    &zbuffer,
                );
                render_minimap(&mut d, &maze, &player, window_width, block_size);

                // FPS counter
                d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::GREEN);
            }
        }
    }
}

