use std::error::Error;
use std::fs::{self, File};
use std::io::{Error as Ioerror, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("hello.txt")?;

    Ok(())
}
fn crushn_burn() {
    panic!("crush and burn"); // use RUST_BACKTRACE=1 to see backtrace
}
fn open_hello_txt_match() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("File opened successfully");
            file
        }

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => {
                    println!("File created successfully!");
                    file
                }

                Err(error) => panic!("Problem creating 'hello.txt': {error}"),
            },

            other_error => panic!("Problem open file:{other_error}"),
        },
    };
}
fn read_username_from_file_long() -> Result<String, Ioerror> {
    let usname_file_result = File::open("hello.txt");

    let mut usname_file = match usname_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut usname = String::new();

    match usname_file.read_to_string(&mut usname) {
        Ok(_) => Ok(usname),
        Err(err) => Err(err),
    }
}
fn read_username_from_file_short() -> Result<String, Ioerror> {
    // let mut usname_file = File::open("hello.txt")?;
    // let mut usname = String::new();
    // usname_file.read_to_string(&mut usname)?;
    // Ok(usname)

    // let mut usname = String::new();
    // File::open("hello.txt")?.read_to_string(&mut usname)?;
    // Ok(usname)

    fs::read_to_string("hello.txt")
}
