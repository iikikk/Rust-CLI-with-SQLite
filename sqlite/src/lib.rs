// use rusqlite::{params, Connection, Result};
// use std::error::Error;  // Import the Error trait
// use csv::Reader;  // Make sure to import the csv Reader
// use std::fs::File; 
// use serde::{Deserialize, Serialize};
// use csv::ReaderBuilder; 
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Movie {
//     pub id: i32,
//     pub title: String,
//     pub director: String,
//     pub release_date: String,
// }
// pub fn create_movie_table(conn: &Connection) -> Result<()> {
//     let create_query = 
//         "CREATE TABLE IF NOT EXISTS movies (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             title TEXT NOT NULL,
//             director TEXT NOT NULL,
//             release_date TEXT NOT NULL
//         )";
//     conn.execute(create_query, [])?;
//     println!("Table 'movies' created successfully.");
//     Ok(())
// }
// pub fn query_movies(conn: &Connection) -> Result<Vec<Movie>> {
//     let mut stmt = conn.prepare("SELECT id, title, director, release_date FROM movies")?;
//     let movies = stmt.query_map([], |row| {
//         Ok(Movie {
//             id: row.get(0)?,
//             title: row.get(1)?,
//             director: row.get(2)?,
//             release_date: row.get(3)?,
//         })
//     })?.collect();
//     movies
// }
// pub fn update_movie(conn: &Connection, id: i32, title: &str, director: &str, release_date: &str) -> Result<usize> {
//     let update_query = format!(
//         "UPDATE movies SET title = ?, director = ?, release_date = ? WHERE id = ?",
//     );
//     let updated_count = conn.execute(
//         &update_query,
//         params![title, director, release_date, id],
//     )?;
//     Ok(updated_count)
// }
// pub fn delete_movie(conn: &Connection, id: i32) -> Result<usize> {
//     let delete_query = "DELETE FROM movies WHERE id = ?";
//     let count = conn.execute(delete_query, params![id])?;
//     println!("Movie with ID {} deleted successfully. Rows affected: {}", id, count);
//     Ok(count)
// }
// pub fn load_data_from_csv(conn: &Connection, table_name: &str, file_path: &str) -> Result<(), RusqliteError> {
//     let file = File::open(file_path)
//         .map_err(|e| RusqliteError::Other(Box::new(e)))?;
//     let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

//     let insert_query = format!(
//         "INSERT INTO {} (title, director, release_date) VALUES (?, ?, ?)",
//         table_name
//     );

//     for result in rdr.records() {
//         let record = result.map_err(|e| RusqliteError::Other(Box::new(e)))?;
//         let title = &record[0];
//         let director = &record[1];
//         let release_date = &record[2];

//         conn.execute(&insert_query, params![title, director, release_date])
//             .map_err(|e| RusqliteError::Other(Box::new(e)))?;
//     }

//     println!(
//         "Data loaded successfully from '{}' into table '{}'.",
//         file_path, table_name
//     );
//     Ok(())
// }

// lib.rs

use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::{ReaderBuilder, WriterBuilder};

/// Represents a movie record.
#[derive(Debug, Clone)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub director: String,
    pub release_date: String,
}

impl Movie {
    /// Creates a new Movie instance.
    pub fn new(id: u32, title: String, director: String, release_date: String) -> Self {
        Self {
            id,
            title,
            director,
            release_date,
        }
    }
}

/// Manages a collection of movies.
pub struct MovieManager {
    pub movies: Vec<Movie>,
}

impl MovieManager {
    /// Initializes a new MovieManager with an empty movie list.
    pub fn new() -> Self {
        Self { movies: Vec::new() }
    }

    /// Loads movie data from a CSV file.
    pub fn load_data_from_csv(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(filepath)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

        for result in rdr.records() {
            let record = result?;
            let id: u32 = record.get(0).unwrap().parse()?;
            let title = record.get(1).unwrap().to_string();
            let director = record.get(2).unwrap().to_string();
            let release_date = record.get(3).unwrap().to_string();

            let movie = Movie::new(id, title, director, release_date);
            self.movies.push(movie);
        }
        Ok(())
    }

    /// Saves movie data to a CSV file.
    pub fn save_data_to_csv(&self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filepath)?;
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);

        wtr.write_record(&["id", "title", "director", "release_date"])?;

        for movie in &self.movies {
            wtr.write_record(&[
                movie.id.to_string(),
                movie.title.clone(),
                movie.director.clone(),
                movie.release_date.clone(),
            ])?;
        }
        wtr.flush()?;
        Ok(())
    }

    /// Adds a new movie to the collection.
    pub fn create_movie(&mut self, id: u32, title: String, director: String, release_date: String) {
        let movie = Movie::new(id, title, director, release_date);
        self.movies.push(movie);
    }

    /// Retrieves a movie by ID.
    pub fn read_movie(&self, id: u32) -> Option<&Movie> {
        self.movies.iter().find(|&m| m.id == id)
    }

    /// Updates an existing movie's information.
    pub fn update_movie(
        &mut self,
        id: u32,
        title: Option<String>,
        director: Option<String>,
        release_date: Option<String>,
    ) -> bool {
        if let Some(movie) = self.movies.iter_mut().find(|m| m.id == id) {
            if let Some(t) = title {
                movie.title = t;
            }
            if let Some(d) = director {
                movie.director = d;
            }
            if let Some(r) = release_date {
                movie.release_date = r;
            }
            true
        } else {
            false
        }
    }

    /// Deletes a movie by ID.
    pub fn delete_movie(&mut self, id: u32) -> bool {
        if let Some(pos) = self.movies.iter().position(|m| m.id == id) {
            self.movies.remove(pos);
            true
        } else {
            false
        }
    }

    /// Lists all movies in the collection.
    pub fn list_movies(&self) {
        for movie in &self.movies {
            println!(
                "ID: {}, Title: {}, Director: {}, Release Date: {}",
                movie.id, movie.title, movie.director, movie.release_date
            );
        }
    }
}
