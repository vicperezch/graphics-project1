use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize,
    pub perpendicular_distance: f32,
}

pub fn cast_ray(maze: &Maze, player: &Player, a: f32, block_size: usize) -> Intersect {
    let mut d = 0.0;
    const MAX_DISTANCE: f32 = 5000.0;
    const STEP_SIZE: f32 = 1.0;

    let cos_a = a.cos();
    let sin_a = a.sin();
    let block_size_f = block_size as f32;

    loop {
        let x = player.pos.x + d * cos_a;
        let y = player.pos.y + d * sin_a;

        let i = (x / block_size_f) as usize;
        let j = (y / block_size_f) as usize;

        if j >= maze.len() || i >= maze[0].len() {
            return Intersect {
                distance: d,
                impact: '#',
                tx: 0,
                perpendicular_distance: d * (a - player.a).cos(),
            };
        }

        if maze[j][i] != ' ' {
            let mut exact_d = d - STEP_SIZE;
            let mut exact_x = player.pos.x + exact_d * cos_a;
            let mut exact_y = player.pos.y + exact_d * sin_a;

            for _ in 0..5 {
                exact_d += 0.2;
                exact_x = player.pos.x + exact_d * cos_a;
                exact_y = player.pos.y + exact_d * sin_a;
                let test_i = (exact_x / block_size_f) as usize;
                let test_j = (exact_y / block_size_f) as usize;
                if test_j < maze.len() && test_i < maze[0].len() && maze[test_j][test_i] != ' ' {
                    break;
                }
            }

            let cell_x = exact_x - (i as f32 * block_size_f);
            let cell_y = exact_y - (j as f32 * block_size_f);

            // Better wall detection and texture coordinate calculation
            let tx = if cell_x <= 1.0 {
                // Left wall
                ((cell_y / block_size_f) * 128.0) as usize
            } else if cell_x >= block_size_f - 1.0 {
                // Right wall
                ((cell_y / block_size_f) * 128.0) as usize
            } else if cell_y <= 1.0 {
                // Top wall
                ((cell_x / block_size_f) * 128.0) as usize
            } else {
                // Bottom wall
                ((cell_x / block_size_f) * 128.0) as usize
            };

            // Calculate perpendicular distance to avoid fisheye
            let angle_diff = a - player.a;
            let perpendicular_distance = exact_d * angle_diff.cos();

            return Intersect {
                distance: exact_d,
                impact: maze[j][i],
                tx: tx.min(127),
                perpendicular_distance,
            };
        }

        d += STEP_SIZE;

        if d > MAX_DISTANCE {
            return Intersect {
                distance: d,
                impact: ' ',
                tx: 0,
                perpendicular_distance: d * (a - player.a).cos(),
            };
        }
    }
}
