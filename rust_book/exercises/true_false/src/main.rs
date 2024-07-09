use rand::Rng;
use std::io;
use std::process;

fn main() {
    println!("Ask questions and i will answer them:\nTrue, False, Maybe, or I dont know");
    println!(">> to exit enter echo string <<");

    run();
}
fn run() {
    let mut response;
    let mut question = String::new();

    loop {
        response = get_response();

        if let Err(e) = read_question(&mut question) {
            eprintln!("Read string error: {e}");

            process::exit(1);
        }
        if question.to_lowercase().trim() == "денис гей?" {
            println!("false");
            continue;
        }

        println!("{response}");
    }
}
fn get_response() -> String {
    let response_gen: u32 = rand::thread_rng().gen_range(0..=3);

    match response_gen {
        0 => "true".to_string(),
        1 => "false".to_string(),
        2 => "maybe".to_string(),
        3 => "i dont know".to_string(),
        _ => panic!("error with response generate: the number should be in 0 to 3 range"),
    }
}
fn read_question(question: &mut String) -> Result<String, io::Error> {
    question.clear();

    io::stdin().read_line(question)?;

    if question == "\n" {
        println!("Bye bye!");

        process::exit(1);
    }

    Ok(question.to_string())
}
