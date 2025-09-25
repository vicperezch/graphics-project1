use raylib::prelude::*;

pub struct WallTextures {
    wall_texture: Vec<Color>,
    enemy_texture: Vec<Color>,
    finish_texture: Vec<Color>,
    texture_size: usize,
    enabled: bool,
    enemy_enabled: bool,
    finish_enabled: bool,
}

impl WallTextures {
    pub fn new() -> Self {
        // Try to load wall texture
        let (wall_texture, texture_size, enabled) =
            if let Ok(image) = Image::load_image("assets/wall.png") {
                println!("Loaded wall texture: {}x{}", image.width, image.height);
                let target_size = (image.width as usize).min(256).max(128);
                let data = Self::extract_colors(&image, target_size);
                (data, target_size, true)
            } else {
                println!("No wall texture found at assets/wall.png - using solid colors");
                (Vec::new(), 128, false)
            };

        // Try to load enemy texture
        let (enemy_texture, enemy_enabled) =
            if let Ok(image) = Image::load_image("assets/enemy.png") {
                println!("Loaded enemy sprite: {}x{}", image.width, image.height);
                let data = Self::extract_colors(&image, texture_size);
                (data, true)
            } else {
                println!("No enemy sprite found at assets/enemy.png - enemies won't be visible");
                (Vec::new(), false)
            };

        // Try to load finish texture
        let (finish_texture, finish_enabled) =
            if let Ok(image) = Image::load_image("assets/finish.png") {
                println!("Loaded finish sprite: {}x{}", image.width, image.height);
                let data = Self::extract_colors(&image, texture_size);
                (data, true)
            } else {
                println!("No finish sprite found at assets/finish.png - using fallback color");
                (Vec::new(), false)
            };

        WallTextures {
            wall_texture,
            enemy_texture,
            finish_texture,
            texture_size,
            enabled,
            enemy_enabled,
            finish_enabled,
        }
    }

    fn extract_colors(image: &Image, target_size: usize) -> Vec<Color> {
        let width = image.width as usize;
        let height = image.height as usize;
        let mut colors = Vec::with_capacity(target_size * target_size);

        if width == target_size && height == target_size {
            unsafe {
                let data_ptr = image.data as *const u8;
                if !data_ptr.is_null() {
                    let data = std::slice::from_raw_parts(data_ptr, width * height * 4);
                    for i in (0..data.len()).step_by(4) {
                        colors.push(Color::new(data[i], data[i + 1], data[i + 2], data[i + 3]));
                    }
                }
            }
        } else {
            for ty in 0..target_size {
                for tx in 0..target_size {
                    let fx = tx as f32 * width as f32 / target_size as f32;
                    let fy = ty as f32 * height as f32 / target_size as f32;
                    let sx = (fx as usize).min(width - 1);
                    let sy = (fy as usize).min(height - 1);

                    let color = unsafe {
                        let data_ptr = image.data as *const u8;
                        if !data_ptr.is_null() {
                            let idx = (sy * width + sx) * 4;
                            let data = std::slice::from_raw_parts(data_ptr, width * height * 4);
                            if idx + 3 < data.len() {
                                Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
                            } else {
                                Color::GRAY
                            }
                        } else {
                            Color::GRAY
                        }
                    };
                    colors.push(color);
                }
            }
        }
        colors
    }

    #[inline(always)]
    pub fn get_pixel(&self, x: usize, y: usize, sprite_type: char) -> Color {
        if sprite_type == 'e' {
            // Enemy sprite
            if !self.enemy_enabled || self.enemy_texture.is_empty() {
                return Color::RED; // Fallback color for enemies
            }
            let tx = (x * self.texture_size / 128).min(self.texture_size - 1);
            let ty = (y * self.texture_size / 128).min(self.texture_size - 1);
            let idx = ty * self.texture_size + tx;

            if idx < self.enemy_texture.len() {
                self.enemy_texture[idx]
            } else {
                Color::RED
            }
        } else if sprite_type == 'w' {
            // Finish/win sprite
            if !self.finish_enabled || self.finish_texture.is_empty() {
                return Color::GOLD; // Fallback color for finish
            }
            let tx = (x * self.texture_size / 128).min(self.texture_size - 1);
            let ty = (y * self.texture_size / 128).min(self.texture_size - 1);
            let idx = ty * self.texture_size + tx;

            if idx < self.finish_texture.len() {
                self.finish_texture[idx]
            } else {
                Color::GOLD
            }
        } else {
            // Wall texture
            if !self.enabled || self.wall_texture.is_empty() {
                return Color::GRAY;
            }
            let tx = (x * self.texture_size / 128).min(self.texture_size - 1);
            let ty = (y * self.texture_size / 128).min(self.texture_size - 1);
            let idx = ty * self.texture_size + tx;

            if idx < self.wall_texture.len() {
                self.wall_texture[idx]
            } else {
                Color::GRAY
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_enemy_enabled(&self) -> bool {
        self.enemy_enabled
    }

    pub fn is_finish_enabled(&self) -> bool {
        self.finish_enabled
    }
}

