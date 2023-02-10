use std::env;
use std::fs::{create_dir_all, OpenOptions};
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
    eprintln!("Usage: todo [COMMAND] [ARGS]...");
    eprintln!("Commands:");
    eprintln!("  add TODOS...    Add to todo list");
    eprintln!("  done INDICES... Remove indices from list");
    eprintln!("  list            Print the list");
    eprintln!("  help            Print this text");
}

fn main() -> Result<(), AppError> {
    let data_dir = if let Ok(path) = env::var("XDG_DATA_HOME") {
        format!("{path}/todo")
    } else {
        let home_folder = env::var("HOME").expect("Failed to get name of home folder");
        format!("{home_folder}/.local/share/todo")
    };

    create_dir_all(&data_dir)?;
    let todo_path = format!("{data_dir}/todo_list.txt");

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
                eprintln!("Failed to read line {e}");
            }
            line.ok()
        })
        .collect();

    let args: Vec<String> = env::args().collect();

    let operation = if let Some(op) = args.get(1) { (*op).clone() } else {
        print_help();
        return Err(AppError::MissingCommand);
    };

    match operation.as_str() {
        "add" => {
            let new_todos = if let Some(a) = args.get(2..) { a } else {
                print_help();
                return Err(AppError::MissingArgs);
            };

            for todo in new_todos {
                todo_list.push(todo.clone());
            }
        }
        "done" => {
            let args: &[String] = if let Some(a) = args.get(2..) { a } else {
                print_help();
                return Err(AppError::MissingArgs);
            };

            let mut indices: Vec<usize> = args
                .iter()
                .filter_map(|str| match str.parse::<usize>() {
                    Ok(0) | Err(_) => {
                        eprintln!("{str} could not be parsed into a positive integer");
                        None
                    }
                    Ok(idx) => Some(idx - 1),
                })
                .collect();

            indices.sort_unstable();
            indices.reverse();

            for idx in indices {
                let done = todo_list.remove(idx);
                println!("DONE!: {done}");
            }
        }
        "list" => {
            // Preprocessing this string seems to be faster than printing each line in a for loop
            let output: String = todo_list
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{} {}\n", i + 1, t))
                .collect::<Vec<String>>()
                .join("");

            print!("{output}");

            return Ok(());
        }
        "help" | "--help" | "-h" => {
            print_help();
            return Ok(());
        }
        _ => {
            print_help();
            return Err(AppError::InvalidCommand);
        }
    };

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(todo_path)
        .expect("Failed to open file for writing");

    let file_content = todo_list.clone().join("\n");
    file.write_all(file_content.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}

mod test {
    #[test]
    fn sanity() {
        assert!(false);
    }
}
