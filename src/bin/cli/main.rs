use clap::Parser;
use co_log;
use directories::ProjectDirs;
use scrawl;
use std::{
    env, fs,
    io::{self, BufRead, Write},
};
use tokio;

mod text;
use text::*;

mod logic_test;

/// The command line arguments.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file to query
    file: Option<String>,

    /// Whether to simply transpile then exit.
    #[arg(short)]
    dry_run: bool,
}

/// Get tne user's input.
fn get_user_input() -> String {
    loop {
        if let Some(line) = io::stdin().lock().lines().nth(0) {
            break line
        }
    }.expect("error reading from stdin")
}

/// Create a file, reading the file name from the keyboard if it is not given.
fn create_file(file_name: Option<String>) -> io::Result<()> {
    let mut chosen_file = "".to_string();

    let project_dirs = ProjectDirs::from("com", "Kleb", "co-log").unwrap();
    let mut user_file_folder = project_dirs.data_dir().to_path_buf();
    user_file_folder.push("user_files");

    if !user_file_folder.exists() {
        println!("doesn't exist");
        fs::create_dir_all(user_file_folder.clone())?;
        println!("created");
    }

    match file_name {
        None => {
            let mut user_files = fs::read_dir(co_log::remove_path_prefix(
                user_file_folder.to_str().unwrap(),
            ))
            .unwrap();

            let mut valid = false;
            while !valid {
                print!("{CREATE_FILE_TEXT}");

                chosen_file = get_user_input();

                if user_files.any(|f| {
                    f.unwrap().path().file_name().unwrap().to_str()
                        == Some(&(chosen_file.clone() + ".cl"))
                }) {
                    println!("File already exists. Would you like to edit this file? Y|N");

                    match get_user_input().to_lowercase().as_str() {
                        "y" => edit_file(Some(chosen_file.clone())),
                        _ => {}
                    }
                } else {
                    valid = true;
                }
            }
        }
        Some(file_name) => chosen_file = file_name,
    }

    user_file_folder.push(chosen_file.clone() + ".cl");

    fs::File::create(user_file_folder.clone())?;

    println!("Would you like to edit this file? Y|N");

    match get_user_input().to_lowercase().as_str() {
        "y" => edit_file(Some(user_file_folder.to_str().unwrap().to_string())),
        _ => {}
    }

    Ok(())
}

/// Gets the name of a given file, not including the file extension.
fn get_file_name(file: &fs::DirEntry) -> String {
    let file_name = file
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let dot = file_name.chars().position(|c| c == '.').unwrap();

    file_name[..dot].to_string()
}

/// Gets the file that the user wants to select.
fn get_file() -> Option<String> {
    let project_dirs = ProjectDirs::from("com", "Kleb", "co-log").unwrap();
    let mut user_file_folder = project_dirs.data_dir().to_path_buf();
    user_file_folder.push("user_files");

    print!("{EDIT_FILE_TEXT}");

    let files = fs::read_dir(co_log::remove_path_prefix(
        user_file_folder.to_str().unwrap(),
    ))
    .unwrap();

    for user_file in files {
        println!("- {}", get_file_name(&user_file.unwrap()));
    }

    println!("");

    let mut files = fs::read_dir(co_log::remove_path_prefix(
        user_file_folder.to_str().unwrap(),
    ))
    .unwrap();

    print!("> ");
    let _ = io::stdout().flush();
    let mut user_file = get_user_input();
    while !files.any(|f| {
        f.unwrap().path().file_name().unwrap().to_str() == Some(&(user_file.clone() + ".cl"))
    }) {
        println!("File not found. Would you like to create this file? Y|N");
        match get_user_input().to_lowercase().as_str() {
            "y" => {
                create_file(Some(user_file.clone())).unwrap();
                return None;
            }
            _ => user_file = get_user_input(),
        }
    }

    user_file_folder.push(user_file + ".cl");

    Some(user_file_folder.to_str().unwrap().to_string())
}

/// Opens a file in the user's preferred text editor, reading the file name from the keyboard if it is not given.
fn edit_file(file_path: Option<String>) {
    let file_to_edit;

    match file_path {
        None => {
            file_to_edit = match get_file() {
                Some(file) => file,
                None => return,
            }
        }
        Some(filename) => {
            file_to_edit = filename;
        }
    }

    scrawl::edit_file(&(file_to_edit)).unwrap();

    println!("Would you like to query the file? Y|N");
    match get_user_input().to_lowercase().as_str() {
        "y" => {
            query_file(Some(file_to_edit));
            return;
        }
        _ => {}
    }
}

/// Queries the selected file, reading the file name from the keyboard if it is not given.
fn query_file(file_path: Option<String>) {
    let file_to_query = match file_path {
        None => match get_file() {
            Some(file) => file,
            None => return,
        },
        Some(file) => file,
    };

    let colog = fs::read_to_string(file_to_query).unwrap();

    let (pl, identifiers) = match co_log::transpile(colog, None) {
        Ok((pl, _, identifiers)) => (pl, identifiers),
        Err(err) => {
            eprintln!("Error: {err}");
            wait_for_input();
            return;
        }
    };

    let mut tmp_location = env::current_exe().expect("failed to get location of executable");
    tmp_location.pop();
    tmp_location.push("temp.pl");

    fs::write(&tmp_location, pl).unwrap();

    /*
    let context = co_log::start_prolog(tmp_location.to_str().unwrap()).unwrap();

    println!("Enter your queries, or enter ':exit' to finish.");

    let mut input = get_user_input();
    while &input != ":exit" {
        let query;
        (query, identifiers) = co_log::transpile_query(input.clone(), identifiers).unwrap();

        let succeeded = co_log::query_prolog(&context, query).unwrap();

        if succeeded {
            println!("Yes");
        } else {
            println!("No");
        }

        input = get_user_input();
    }

    eprintln!("error: implementation not finished")
    */

    println!(
        "The generated code can be found at {}. The following table can be used to translate Prolog's responses:",
        tmp_location.display(),
    );

    println!("{:<15} | {:<15}", "Co-log name", "Prolog name");
    println!("{}", "_".repeat(16) + "|" + &"_".repeat(16));
    for identifier in identifiers.identifiers() {
        if identifier.cl_name() == "eq" {
            println!("{:<15} | {:<15}", "is", "eq");
        } else {
            println!(
                "{:<15} | {:<15}",
                identifier.cl_name(),
                identifier.pl_name()
            );
        }
    }
}

fn display_menu() {
    print!("{MAIN_MENU_TEXT}");
    print!("> ");
    let _ = io::stdout().flush();
}

fn wait_for_input() {
    println!("\nPress enter to continue...");
    get_user_input();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(file) = args.file {
        if args.dry_run {
            co_log::transpile(co_log::read_file(&file).unwrap(), None).unwrap();
        } else {
            query_file(Some(file));
        }
    } else {
        let _ = io::stdout().flush();
        display_menu();
        loop {
            match get_user_input().to_lowercase().as_str() {
                "i" => {
                    print!("{INTRO_TEXT}");
                    wait_for_input();
                    display_menu();
                }
                "c" => {
                    create_file(None).unwrap();
                    display_menu();
                }
                "e" => {
                    edit_file(None);
                    display_menu();
                }
                "q" => {
                    query_file(None);
                    display_menu();
                }
                "t" => {
                    if let Err(err) = logic_test::test().await {
                        eprintln!("Error: {err}");
                    }
                    wait_for_input();
                    display_menu();
                }
                "x" => return,
                _ => {
                    println!("Unrecognised input");
                    print!("> ");
                    let _ = io::stdout().flush();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
