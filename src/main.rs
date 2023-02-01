use std::env;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, prelude::*, BufRead, BufReader};

#[derive(Debug)]
enum AppError {
    MissingCommand,
    MissingArgs,
    InvalidCommand,
    IOError(io::Error),
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::IOError(e)
    }
}

fn print_help() {
    println!("Usage: todo [COMMAND] [ARGS]...");
    println!("Commands:");
    println!("  add TODOS...    Add to todo list");
    println!("  done INDICES... Remove indices from list");
    println!("  list            Print the list");
    println!("  help            Print this text");
}

fn main() -> Result<(), AppError> {
    let data_dir = match env::var("XDG_DATA_HOME") {
        Ok(path) => format!("{}/todo", path),
        Err(_) => {
            let home_folder = env::var("HOME").expect("Failed to get name of home folder");
            format!("{}/.local/share/todo", home_folder)
        }
    };

    create_dir_all(&data_dir)?;
    let todo_path = format!("{}/todo_list.txt", data_dir);

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&todo_path)
        .expect("Failed to open file for writing");

    let mut todo_list: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|line| {
            if let Err(ref e) = line {
                println!("Failed to read line {}", e);
            }
            line.ok() 
        })
        .collect();

    let args: Vec<String> = env::args().collect();

    let operation = match args.get(1) {
        Some(op) => (*op).clone(),
        None => {
            print_help();
            return Err(AppError::MissingCommand);
        }
    };

    match operation.as_str() {
        "add" => {
            let new_todos = match args.get(2..) {
                Some(args) => args,
                None => {
                    print_help();
                    return Err(AppError::MissingArgs);
                }
            };

            for todo in new_todos { 
                todo_list.push(todo.clone());
            }
        },
        "done" => {
            let args: &[String] = match args.get(2..) {
                Some(args) => args,
                None => {
                    print_help();
                    return Err(AppError::MissingArgs);
                }
            };

            let mut indices: Vec<usize> = args
                .iter()
                .filter_map(|str| match str.parse::<usize>() {
                    Ok(0) | Err(_) => {
                        println!("{} could not be parsed into a positive integer", str);
                        None
                    },
                    Ok(idx) => Some(idx - 1),
                })
                .collect();

            indices.sort();
            indices.reverse();

            for idx in indices {
                let done = todo_list.remove(idx);
                println!("DONE!: {}", done);
            }
        },
        "list" => {
        // Preprocessing this string seems to be faster than printing each line in a for loop
            let output: String = todo_list
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{} {}", i + 1, t))
                .collect::<Vec<String>>()
                .join("\n");

            println!("{}", output);

            return Ok(())
        },
        "help" | "--help" | "-h" => {
            print_help();
            return Ok(());
        },
        _ => {
            print_help();
            return Err(AppError::InvalidCommand)
        }
    };

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(todo_path)
        .expect("Failed to open file for writing");

    let file_content = todo_list.clone().join("\n");
    file.write_all(file_content.as_bytes()).expect("Failed to write to file");

    Ok(())
}


mod test {
    #[test]
    fn sanity() {
        assert!(false);
    }
}
