use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::{self, read_to_string, File};
use std::io::{self, ErrorKind, Write};
use std::process;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}
impl User {
    fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
        }
    }
}

fn main() {
    let storage_path = "storage.txt";
    let storage_file = open_storage(&storage_path);

    println!("Hello! This is program for authorization");

    if let Ok(s) = fs::read_to_string(storage_path) {
        if s.is_empty() {
            first_time(&storage_file);
        } else {
            not_first_time(storage_path, &storage_file);
        }
    };
}
fn first_time(storage_file: &File) -> User {
    println!("This is your first time here, so register.");

    let user = user_registration(&storage_file);

    user
}
fn not_first_time(storage_path: &str, storage_file: &File) {
    let mut log_or_reg = String::new();

    println!("You already have acconut. You want login or registration?");
    println!("(enter 'log' or 'reg')");

    loop {
        io::stdin()
            .read_line(&mut log_or_reg)
            .expect("Error to read user input in func 'not_first_time'");

        if log_or_reg.to_lowercase().trim().contains("reg") {
            user_registration(storage_file);
            break;
        } else if log_or_reg.to_lowercase().trim().contains("log") {
            if let Err(e) = user_login(storage_path) {
                println!("File to login: {e}!");
            }
            break;
        } else {
            println!("Please, enter 'log' or 'reg'");
            continue;
        }
    }
}
fn user_registration(storage_file: &File) -> User {
    let user = write_user_to_storage(storage_file);

    println!(
        "You sucessfully registred!\nYour username: {}, your password: {}",
        user.username, user.password
    );

    user
}
fn if_contains<'a, T>(what_find: String, where_find: String, f: T) -> Result<Vec<String>, &'a str>
where
    T: FnOnce(),
{
    let lines: Vec<_> = where_find
        .lines()
        .filter(|line| line.contains(&what_find))
        .map(|str| str.to_string())
        .collect();

    if !lines.is_empty() {
        f();

        Ok(lines)
    } else {
        Err("not contains")
    }
}
fn user_login(storage_path: &str) -> Result<User, &str> {
    let from_user_input: User = read_user();
    let check_user_json =
        serde_json::to_string(&from_user_input).expect("Error to convert to json");

    let storage_str = read_to_string(storage_path).expect("Error to read file data");

    match if_contains(check_user_json, storage_str, || {
        println!("You succesfully login in!");
    }) {
        Ok(_) => Ok(from_user_input),
        Err(_) => Err("incorrect username/password.
(if you forgot that, you can check file 'storage.txt'\n in current directory)"),
    }
}
fn write_user_to_storage(mut storage_file: &File) -> User {
    let user = read_user();

    serde_json::to_writer(storage_file, &user).expect("Error to write json in file");
    storage_file
        .write(b"\n")
        .expect("Error to write '\\n' in storage file");

    user
}
fn read_user() -> User {
    let mut user = User::new();

    user.username = read_username();
    user.password = read_password();

    user
}
fn read_username() -> String {
    let mut username = String::new();

    println!("Enter username:");

    io::stdin()
        .read_line(&mut username)
        .expect("Error to read username");
    username = username.trim().to_string();

    username
}
fn read_password() -> String {
    let mut password = String::new();

    println!("Enter password:");

    io::stdin()
        .read_line(&mut password)
        .expect("Error to read password");
    password = password.trim().to_string();

    password
}
fn open_storage(storage_path: &str) -> File {
    let _file = open_or_create(storage_path).unwrap_or_else(|e| {
        eprintln!("Error processing file: {e}");

        process::exit(1);
    });

    let storage = File::options()
        .append(true)
        .open(storage_path)
        .expect("Error to open file option");

    storage
}
fn open_or_create(storage_path: &str) -> Result<File, io::Error> {
    match File::open(storage_path) {
        Ok(f) => Ok(f),

        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create(storage_path),

            other_err => Err(other_err.into()),
        },
    }
}
