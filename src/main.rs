/*
By: <Abdul baig>
Date: 2026-11-16
Program Details: <Database Setup: Create an SQLite3 database and a table to store movie information.
Insert Data: Allow the user to insert new movie records into the table.
Query Data: Allow the user to query all movie records from the table.
Update Data: Allow the user to update the information of an existing movie record.
Delete Data: Allow the user to delete a movie record from the table.
Display Data: Use rust to display the movie records.>
*/




mod ui;
mod utils;

use macroquad::{color, prelude::*};
use crate::ui::still_image::StillImage;
 use crate::ui::text_input::TextInput;
 use crate::utils::preload_image::TextureManager;
 use crate::ui::grid::draw_grid;
    use crate::ui::label::Label;
    use crate::ui::listview::ListView;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "movie".to_string(),
        window_width: 1920,
        window_height: 900,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {


let tm = TextureManager::new();
//  tm.preload_with_loading_screen(&[""], None, None).await;

     let lbl_movie = Label::new("MOVIE", 50.0, 100.0, 30);
     let lbl_movie_ex = Label::new("movie (year),genres", 50.0, 100.0, 30);
  let mut txt_movie = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);

       let lbl_rating = Label::new("RATING", 50.0, 100.0, 30);
       let lbl_rating_ex = Label::new("(0-10)", 50.0, 100.0, 30);
   let mut txt_rating = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);

 let lbl_genres = Label::new("Genres", 50.0, 100.0, 30);
       let lbl_genre_ex = Label::new("(0-10)", 50.0, 100.0, 30);
   let mut txt_genre = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);


let movies = vec![""];
    let mut list_movies = ListView::new(&movies, 100.0, 100.0, 50);


let rating = vec![""];
    let mut list_rating = ListView::new(&rating,100.0, 100.0, 50);


let genres = vec![""];
    let mut list_genre = ListView::new(&genres,100.0, 100.0, 50);


    loop {                          
   clear_background(DARKBLUE);
       draw_grid(50.0, BLACK);   

    



list_genre.draw();
list_rating.draw();
list_movies.draw();
txt_movie.draw();
lbl_movie.draw();
lbl_rating.draw();
txt_rating.draw();
        next_frame().await;
    }
}
