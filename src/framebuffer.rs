use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        // Only regenerate if absolutely necessary
        // For now, we'll use the existing clear method but could optimize with unsafe direct memory access
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: i32, y: i32) {
        // Inline for better performance and remove redundant bounds checking
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        // Get FPS before borrowing window mutably
        let fps = window.get_fps();

        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);

            // Draw FPS counter
            renderer.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::GREEN);
        }
    }
}

