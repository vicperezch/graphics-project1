use raylib::prelude::*;

pub struct WallTextures {
    texture_data: Vec<Color>,
    texture_size: usize,
    enabled: bool,
}

impl WallTextures {
    pub fn new() -> Self {
        // Try to load texture from file
        let (texture_data, texture_size, enabled) =
            if let Ok(image) = Image::load_image("assets/wall.png") {
                println!("Loaded wall texture: {}x{}", image.width, image.height);

                // Use native resolution up to 256x256 for better quality
                let target_size = (image.width as usize).min(256).max(128);
                let data = Self::extract_colors(&image, target_size);
                (data, target_size, true)
            } else {
                println!("No wall texture found at assets/wall.png - using solid colors");
                (Vec::new(), 128, false)
            };

        WallTextures {
            texture_data,
            texture_size,
            enabled,
        }
    }

    fn extract_colors(image: &Image, target_size: usize) -> Vec<Color> {
        let width = image.width as usize;
        let height = image.height as usize;
        let mut colors = Vec::with_capacity(target_size * target_size);

        // If image is already the target size, use it directly
        if width == target_size && height == target_size {
            // Direct copy for better quality
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
            // Scale image to target size
            for ty in 0..target_size {
                for tx in 0..target_size {
                    // Bilinear interpolation for smoother scaling
                    let fx = (tx as f32 * width as f32 / target_size as f32);
                    let fy = (ty as f32 * height as f32 / target_size as f32);

                    let sx = fx as usize;
                    let sy = fy as usize;

                    let sx = sx.min(width - 1);
                    let sy = sy.min(height - 1);

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
    pub fn get_pixel(&self, x: usize, y: usize, _wall_type: char) -> Color {
        if !self.enabled || self.texture_data.is_empty() {
            return Color::GRAY;
        }

        // Scale coordinates to actual texture size
        let tx = (x * self.texture_size / 128).min(self.texture_size - 1);
        let ty = (y * self.texture_size / 128).min(self.texture_size - 1);
        let idx = ty * self.texture_size + tx;

        if idx < self.texture_data.len() {
            self.texture_data[idx]
        } else {
            Color::GRAY
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
