#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
enum Message {
    Quit,
    Hello,
    Bye,
}
impl Message {
    fn sure_quit() {
        println!("r you want to quit?");
        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Error with reading");

        match choice.as_str().trim() {
            "n" => {
                println!("Thanks!");
                return;
            }
            "no" => {
                println!("Thanks!");
                return;
            }
            _ => (),
        };

        println!("r you sure want to quit? :(");
        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Error with reading");

        match choice.as_str().trim() {
            "y" => panic!("Segmentation fault"),
            "yes" => panic!("Segmentation fault"),

            _ => {
                println!("Thanks1");
                return;
            }
        }
    }
}

fn main() {
    show_kitty();

    println!("bye!");
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
fn plus_one(num: Option<i32>) -> Option<i32> {
    num.and_then(|num| Some(num + 1))
}
fn message(option: Message) {
    match option {
        Message::Hello => println!("Hello!"),

        Message::Bye => println!("Bye("),

        Message::Quit => Message::sure_quit(),
    }
}
enum Kitty {
    KittyWithGlasses,
    Kitty,
}

impl Kitty {
    fn from_choice(num: u8) -> Self {
        match num {
            1 => Kitty::KittyWithGlasses,
            2 => Kitty::Kitty,
            _ => panic!("Func: from_choice(): error"),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Kitty::KittyWithGlasses => {
                " /-\\____/-\\\n\
                  |\\ ________|\n\
                  | \\||/ \\||/|\n\
                  |    _|_   |\n"
            }
            Kitty::Kitty => {
                " /-\\____/-\\\n\
                  |          |\n\
                  |   |   |  |\n\
                  |    _|_   |\n"
            }
        }
    }
}

fn show_kitty() {
    let choice = chioce_kitty();
    let kitty = Kitty::from_choice(choice);

    println!("{}", kitty.as_str());
}
fn chioce_kitty() -> u8 {
    let mut choice = String::new();

    loop {
        choice.clear();

        println!("Enter:\n1(cat with glasses) or 2(cat without glasses)");

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Error with input reading");

        match choice.trim().parse() {
            Ok(one) => match one {
                1..=2 => return one,
                _ => {
                    println!("{}", one);

                    continue;
                }
            },
            Err(_) => {
                continue;
            }
        };
    }
}
