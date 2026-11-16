





use macroquad::prelude::*;

use crate::ui::still_image::StillImage;
use crate::ui::text_input::TextInput;
use crate::utils::preload_image::TextureManager;

pub async fn run() -> String {
    
    
    
    
    loop {
        clear_background(BLUE);
        draw_text("Screen 1", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "main".to_string();
        }

        next_frame().await;
    }
}