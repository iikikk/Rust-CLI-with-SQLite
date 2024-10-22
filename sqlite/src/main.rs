// use clap::{Parser, Subcommand};
// use rusqlite::{Connection, Result as SqliteResult};

// // Import your library functions here. Assuming they are in the `lib` module.
// use sqlite::{create_movie_table, query_movies, update_movie, delete_movie, load_data_from_csv};

// #[derive(Parser, Debug)]
// #[command(name = "rust-cli")]
// #[command(about = "A simple CLI for managing a movies database", long_about = None)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand, Debug)]
// enum Commands {
//     /// Pass a table name to create a table
//     #[command(alias = "c", short_flag = 'c')]
//     Create { table_name: String },
//     /// Query to list all movies
//     #[command(alias = "q", short_flag = 'q')]
//     Query {},
//     /// Update a movie's details by ID
//     #[command(alias = "u", short_flag = 'u')]
//     Update {
//         id: i32,
//         title: String,
//         director: String,
//         release_date: String,
//     },
//     /// Delete a movie by ID
//     #[command(alias = "d", short_flag = 'd')]
//     Delete { id: i32 },
//     /// Load movie data from a CSV file
//     #[command(alias = "l", short_flag = 'l')]
//     Load {
//         table_name: String,
//         file_path: String,
//     },
// }

// fn main() -> SqliteResult<()> {
//     let cli = Cli::parse();
//     let conn = Connection::open("data/movies.db")?;

//     match cli.command {
//         Commands::Create { table_name } => {
//             println!("Creating table '{}'", table_name);
//             // Assuming create_movie_table does not actually use table_name, adjust if needed
//             create_movie_table(&conn)?;
//             println!("Table created successfully.");
//         },
//         Commands::Query {} => {
//             let movies = query_movies(&conn)?;
//             println!("Movies: {:?}", movies);
//         },
//         Commands::Update { id, title, director, release_date } => {
//             update_movie(&conn, id, &title, &director, &release_date)?;
//             println!("Movie with ID {} updated successfully.", id);
//         },
//         Commands::Delete { id } => {
//             delete_movie(&conn, id)?;
//             println!("Movie with ID {} deleted successfully.", id);
//         },
//         Commands::Load { table_name, file_path } => {
//             load_data_from_csv(&conn, &table_name, &file_path)?;
//             println!("Data loaded successfully from '{}'", file_path);
//         },
//     }

//     Ok(())
// }
// main.rs

use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

// mod lib; // Import lib.rs
// use sqlite::{Movie, MovieManager};
// use lib::MovieManager;
use sqlite::MovieManager; 
/// Enum representing available commands.
enum Command {
    Create,
    Read,
    Update,
    Delete,
    List,
    Help,
}

impl Command {
    /// Converts a string to a Command enum variant.
    fn from_str(input: &str) -> Option<Command> {
        match input.to_lowercase().as_str() {
            "create" => Some(Command::Create),
            "read" => Some(Command::Read),
            "update" => Some(Command::Update),
            "delete" => Some(Command::Delete),
            "list" => Some(Command::List),
            "help" => Some(Command::Help),
            _ => None,
        }
    }
}

/// Prints help information.
fn print_help() {
    println!("Usage:");
    println!("    tool_name <command> [arguments]");
    println!();
    println!("Commands:");
    println!("    create <id> <title> <director> <release_date>");
    println!("    read <id>");
    println!("    update <id> [title] [director] [release_date]");
    println!("    delete <id>");
    println!("    list");
    println!("    help");
}

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a command is provided
    if args.len() < 2 {
        eprintln!("Error: No command provided.");
        print_help();
        process::exit(1);
    }

    // Parse the command
    let command_str = &args[1];
    let command = match Command::from_str(command_str) {
        Some(cmd) => cmd,
        None => {
            eprintln!("Error: Unknown command '{}'", command_str);
            print_help();
            process::exit(1);
        }
    };

    let mut manager = MovieManager::new();

    // Load data from CSV
    let data_file = "../data/movies.csv";
    if Path::new(data_file).exists() {
        manager.load_data_from_csv(data_file)?;
    }

    // Match the command and execute the corresponding action
    match command {
        Command::Create => {
            if args.len() != 6 {
                eprintln!("Usage: create <id> <title> <director> <release_date>");
                process::exit(1);
            }
            let id: u32 = args[2].parse()?;
            let title = args[3].clone();
            let director = args[4].clone();
            let release_date = args[5].clone();
            manager.create_movie(id, title, director, release_date);
            manager.save_data_to_csv(data_file)?;
            println!("Movie created.");
        }
        Command::Read => {
            if args.len() != 3 {
                eprintln!("Usage: read <id>");
                process::exit(1);
            }
            let id: u32 = args[2].parse()?;
            if let Some(movie) = manager.read_movie(id) {
                println!(
                    "ID: {}, Title: {}, Director: {}, Release Date: {}",
                    movie.id, movie.title, movie.director, movie.release_date
                );
            } else {
                println!("Movie with ID {} not found.", id);
            }
        }
        Command::Update => {
            if args.len() < 3 {
                eprintln!("Usage: update <id> [title] [director] [release_date]");
                process::exit(1);
            }
            let id: u32 = args[2].parse()?;
            let title = args.get(3).cloned();
            let director = args.get(4).cloned();
            let release_date = args.get(5).cloned();

            if manager.update_movie(id, title, director, release_date) {
                manager.save_data_to_csv(data_file)?;
                println!("Movie updated.");
            } else {
                println!("Movie with ID {} not found.", id);
            }
        }
        Command::Delete => {
            if args.len() != 3 {
                eprintln!("Usage: delete <id>");
                process::exit(1);
            }
            let id: u32 = args[2].parse()?;
            if manager.delete_movie(id) {
                manager.save_data_to_csv(data_file)?;
                println!("Movie deleted.");
            } else {
                println!("Movie with ID {} not found.", id);
            }
        }
        Command::List => {
            manager.list_movies();
        }
        Command::Help => {
            print_help();
        }
    }

    Ok(())
}
