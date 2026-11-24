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


use crate::utils::database::{DatabaseTable, create_database_client};
use macroquad::{color, prelude::*};
use crate::ui::still_image::StillImage;
 use crate::ui::text_input::TextInput;
 use crate::utils::preload_image::TextureManager;
 use crate::ui::grid::draw_grid;
    use crate::ui::label::Label;
    use crate::ui::listview::{self, ListView};
    use crate::ui::text_button::TextButton;
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

   

  let btn_add = TextButton::new(50.0, 450.0, 100.0, 60.0, "Add", BLUE, BLACK, 30);

 let btn_update = TextButton::new(250.0, 450.0, 100.0, 60.0, "Update", BLUE, BLACK, 30);

 let btn_delete = TextButton::new(100.0, 560.0, 200.0, 60.0, "Delete", BLUE, BLACK, 30);


      let mut lbl_id = Label::new("Id (for deleting only)", 50.0, 100.0, 30);
    // lbl_movie.with_colors(BLACK, Some(DARKGRAY));
    let mut txt_id: TextInput = TextInput::new(50.0, 100.0, 300.0, 40.0, 25.0);

     let mut lbl_movie = Label::new("MOVIE", 50.0, 200.0, 30);
    // lbl_movie.with_colors(BLACK, Some(DARKGRAY));
    let mut txt_movie = TextInput::new(50.0, 200.0, 300.0, 40.0, 25.0);
    
    let mut lbl_rating = Label::new("RATING", 50.0, 300.0, 30);
    let mut txt_rating = TextInput::new(50.0, 300.0, 300.0, 40.0, 25.0);
    
    let lbl_genres = Label::new("Genres", 50.0, 400.0, 30);
    let mut txt_genre = TextInput::new(50.0, 400.0, 300.0, 40.0, 25.0);
    

  let mut lbl_searchbar = Label::new("Id (for deleting only)", 580.0, 100.0, 30);
    // lbl_movie.with_colors(BLACK, Some(DARKGRAY));
    let mut txt_searchbar: TextInput = TextInput::new(50.0, 100.0, 300.0, 40.0, 25.0);


    
      let client = create_database_client();

 let lbl_sections = Label::new("| Title: | Rating: | Genres: |", 580.0, 150.0, 40);
 let mut movies = vec![];
    let mut list_view = ListView::new(&movies, 580.0, 210.0, 34);
    list_view.set_width(550.0);

    list_view.with_colors(WHITE, Some(BLACK), Some(BLUE));
    list_view.with_max_visible_items(9);

    


    
    //    let lbl_rating_ex = Label::new("(0-10)", 50.0, 100.0, 30);
    //  let mut lbl_movie_ex = Label::new("movie (year),genres", 50.0, 100.0, 30);
    // let lbl_genre_ex = Label::new("(0-10)", 50.0, 100.0, 30);




        let client = create_database_client();
    loop {                          
   clear_background(WHITE);
       draw_grid(50.0, BLACK);   

       let mut records: Vec<DatabaseTable> = Vec::new();
            let fetched_results = client.fetch_table("MOVIE").await;
            if let Ok(result) = fetched_results {
                records = result;
            }
        for record in &records {
                movies.push(format!(
                "| {} | {} | {} |", record.movie, record.rating, record.genre));
                list_view.clear();
                list_view.add_items(&movies);
            }

    // by movie names instead of ID
       if btn_delete.click() {
    //        let mut records: Vec<DatabaseTable> = Vec::new();
    //        let fetched_results = client.fetch_table("MOVIE").await;
    //        if let Ok(result) = fetched_results {
    //            records = result;
    //        }
    //        for i in &records {
    //            println!("movie: {}, rating: {}, genre: {}", i.movie, i.rating, i.genre);
    //            if txt_movie.get_text() == i.movie && txt_rating.get_text() == i.rating && txt_genre.get_text() == i.genre {
    //                if let Ok(deleted_count) = client.delete_record_by_string("MOVIE", i.movie.clone()).await {
    //                    // deleted_count is the number of records deleted
    //                } else {
    //                    // Handle error
    //                }
    //            }
    //        }
    //    

         let mut records: Vec<DatabaseTable> = Vec::new();
           let fetched_results = client.fetch_table("MOVIE").await;
           if let Ok(result) = fetched_results {
               records = result;
           }
           for i in &records {
               println!(" movie: {}, rating: {}, genre: {}", i.movie, i.rating, i.genre);
               if txt_movie.get_text() == i.movie && txt_rating.get_text() == i.rating && txt_genre.get_text() == i.genre {
                   if let Ok(deleted_count) = client.delete_record_by_id("MOVIE", i.id as i64).await {
                       // deleted_count is the number of records deleted
                   } else {
                       // Handle error
                   }
               }
           }
       }
       


if btn_add.click() {
            // Insert a record (from user text input)
            let new_record = DatabaseTable {
                id: 0,
                movie: txt_movie.get_text().to_string(),
                rating: txt_rating.get_text().to_string(),
                 genre: txt_genre.get_text().to_string(),
            };
        }

          if btn_update.click() {
           let mut records: Vec<DatabaseTable> = Vec::new();
           let fetched_results = client.fetch_table("MOVIE").await;
           if let Ok(result) = fetched_results {
               records = result;
           }
           for i in &records {
               println!("id {}, Movie: {}, Rating: {}, Genre: {}", i.id, i.movie, i.rating, i.genre);
               if txt_movie.get_text() == i.movie && txt_rating.get_text() == i.rating && txt_genre.get_text() == i.genre {
                   if let Ok(updated_count) = client.update_record_by_id
                   ("MOVIE", i.id as i64, "Movie", &txt_movie.get_text()).await {
                       // updated_count is the number of records updated
                   } else {
                       // Handle error
                   }
                   if let Ok(updated_count) = client.update_record_by_id
                   ("MOVIE", i.id as i64, "Rating", &txt_rating.get_text()).await {
                       // updated_count is the number of records updated
                
                    if let Ok(updated_count) = client.update_record_by_id
                   ("MOVIE", i.id as i64, "Genre", &txt_genre.get_text()).await {
                       // updated_count is the number of records updated
                   }
                   } else {
                       // Handle error
                   }
               }
           }
       }


lbl_sections.draw();
list_view.draw();
// list_id.draw();
txt_id.draw();
lbl_id.draw();
// list_movies.draw();
txt_movie.draw();
lbl_movie.draw();
lbl_rating.draw();
txt_rating.draw();
// list_rating.draw();
// list_genre.draw();
lbl_genres.draw();
txt_genre.draw();
        next_frame().await;
    }
}
