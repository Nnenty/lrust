use std::io::stdin;

fn main() {
    let mut degrees_in_cels: f32;

    loop {
        degrees_in_cels = degrees_input();

        println!(
            "{} °C = {}° Fahrenheit",
            degrees_in_cels,
            degrees_conversion(degrees_in_cels)
        );
    }
}
fn degrees_input() -> f32 {
    println!("Please, enter degrees Celsius to convert to Fahrenheit:(enter q to exit)");

    let mut cels: String = String::new();

    stdin()
        .read_line(&mut cels)
        .expect("Error with read from stdin");

    let cels: f32 = match cels.trim().parse() {
        Ok(cels) => cels,
        Err(_) => std::process::exit(0),
    };

    cels
}
fn degrees_conversion(cels: f32) -> f32 {
    let fahr = (cels * 9f32 / 5f32) + 32f32;

    fahr
}
