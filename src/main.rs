/*
By: <Abdul baig>
Date: 2026-11-16
Program Details: <Program Description Here>
*/


pub mod menu;

mod ui;
mod utils;

use macroquad::prelude::*;
use crate::ui::still_image::StillImage;
 use crate::ui::text_input::TextInput;
 use crate::utils::preload_image::TextureManager;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "movie".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
   
   
   
   
   
   
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "menu" => menu::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
