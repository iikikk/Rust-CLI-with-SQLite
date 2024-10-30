use std::env;
use std::error::Error;
use std::process;
use sqlite::MovieManager;

/// 枚举，表示可用的命令。
enum Command {
    Create,
    Read,
    Update,
    Delete,
    List,
    Help,
}

impl Command {
    /// 将字符串转换为命令枚举。
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

/// 打印帮助信息。
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
    // 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了命令
    if args.len() < 2 {
        eprintln!("Error: No command provided.");
        print_help();
        process::exit(1);
    }

    // 解析命令
    let command_str = &args[1];
    let command = match Command::from_str(command_str) {
        Some(cmd) => cmd,
        None => {
            eprintln!("Error: Unknown command '{}'", command_str);
            print_help();
            process::exit(1);
        }
    };

    // 初始化电影管理器
    let manager = MovieManager::new("movies.db")?;

    // 根据命令执行相应的操作
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
            manager.create_movie(id, title, director, release_date)?;
            println!("Movie created.");
        }
        Command::Read => {
            if args.len() != 3 {
                eprintln!("Usage: read <id>");
                process::exit(1);
            }
            let id: u32 = args[2].parse()?;
            if let Some(movie) = manager.read_movie(id)? {
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

            if manager.update_movie(id, title, director, release_date)? {
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
            if manager.delete_movie(id)? {
                println!("Movie deleted.");
            } else {
                println!("Movie with ID {} not found.", id);
            }
        }
        Command::List => {
            manager.list_movies()?;
        }
        Command::Help => {
            print_help();
        }
    }

    Ok(())
}
