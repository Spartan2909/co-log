use cl_core;
use clap::Parser;
use directories::ProjectDirs;
use scrawl;
use std::{
    env, fs,
    io::{self, BufRead},
};
use tokio;

mod text;
use text::*;

mod logic_test;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file to query
    file: Option<String>,
}

fn get_user_input() -> String {
    std::io::stdin()
        .lock()
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .to_lowercase()
}

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
            let mut user_files = fs::read_dir(cl_core::remove_path_prefix(
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

fn get_file_name(file: fs::DirEntry) -> String {
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

fn get_file() -> Option<String> {
    let project_dirs = ProjectDirs::from("com", "Kleb", "co-log").unwrap();
    let mut user_file_folder = project_dirs.data_dir().to_path_buf();
    user_file_folder.push("user_files");

    print!("{EDIT_FILE_TEXT}");

    let files = fs::read_dir(cl_core::remove_path_prefix(
        user_file_folder.to_str().unwrap(),
    ))
    .unwrap();

    for user_file in files {
        println!("- {}", get_file_name(user_file.unwrap()));
    }

    println!("");

    let mut files = fs::read_dir(cl_core::remove_path_prefix(
        user_file_folder.to_str().unwrap(),
    ))
    .unwrap();

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

fn query_file(file_path: Option<String>) {
    let file_to_query = match file_path {
        None => match get_file() {
            Some(file) => file,
            None => return,
        },
        Some(file) => file,
    };

    let colog = fs::read_to_string(file_to_query).unwrap();

    let pl = cl_core::transpile(colog).unwrap();

    let mut tmp_location = env::current_exe().unwrap();
    tmp_location.pop();
    tmp_location.push("temp.pl");

    fs::write(tmp_location.clone(), pl.0).unwrap();

    let context = cl_core::start_prolog(tmp_location.to_str().unwrap()).unwrap();

    println!("Enter your queries, or enter ':exit' to finish.");

    let mut input = get_user_input();
    while &input != ":exit" {
        let query = cl_core::transpile_query(input.clone()).unwrap();

        let succeeded = cl_core::query_prolog(&context, query).unwrap();

        if succeeded {
            println!("Yes");
        } else {
            println!("No");
        }

        input = get_user_input();
    }

    eprintln!("error: implementation not finished")
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();

    if let Some(file) = args.file {
        query_file(Some(file));

        Ok(())
    } else {
        print!("{MAIN_MENU_TEXT}");
        loop {
            match get_user_input().to_lowercase().as_str() {
                "c" => {
                    create_file(None).unwrap();
                    print!("{MAIN_MENU_TEXT}");
                }
                "e" => {
                    edit_file(None);
                    print!("{MAIN_MENU_TEXT}");
                }
                "q" => {
                    query_file(None);
                    print!("{MAIN_MENU_TEXT}");
                }
                "t" => {
                    logic_test::test().await?;
                    print!("{MAIN_MENU_TEXT}");
                }
                "x" => return Ok(()),
                _ => println!("Unrecognised input"),
            }
        }
    }
}

#[cfg(test)]
mod tests;
