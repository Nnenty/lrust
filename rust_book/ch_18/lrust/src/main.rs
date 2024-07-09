use learn_base_about_patterns::{let_expressions, matches};
mod learn_base_about_patterns {
    pub fn matches() {
        let x = Some(3);
        let x: Option<i32> = None;

        match x {
            Some(x) => println!("x = {x}"),
            None => println!("x has None value"),
        }
    }
    pub fn let_expressions() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u32, _> = "".parse();

        if let Some(c) = favorite_color {
            println!("using {c} color");
        } else if is_tuesday {
            println!("using green color because its tuesday!");
        } else if let Ok(age) = age {
            if age >= 30 {
                println!("using yellow color");
            } else if age <= 30 {
                println!("using purple color");
            }
        } else {
            println!("usings blue color");
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        let mut stack = Vec::new();

        stack.push("hello");
        stack.push("world");

        while let Some(w) = stack.pop() {
            println!("{}", w);
        }
    }
}
use learn_refutability::refutability;
mod learn_refutability {
    pub fn refutability() {
        let x = Some(5);

        if let Some(n) = x {
            println!("x = {n}");
        }
        if let x = 7 {
            println!("another x = {x}");
        }
    }
}
use learn_patterns_syntax::{
    destructuring, ignoring_patterns_value, match_guards, multi_patterns, ranges,
};
mod learn_patterns_syntax {
    pub fn multi_patterns() {
        let x = 2;

        match x {
            1 | 2 => println!("x = one or two"),
            3 => println!("x = three"),
            _ => println!("x = anything number"),
        };
    }
    pub fn ranges() {
        let x = 5;

        match x {
            3..=6 => println!("x = three through six"),
            1 | 3 => println!("x = one or three"),
            _ => println!("x = anything number"),
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        let x = 'f';

        match x {
            'a'..='h' => println!("x = a through h in ASCII"),
            _ => println!("x = something symbol"),
        }
    }
    pub fn destructuring() {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 0 };

        match p {
            Point { x: 0, y: _ } => println!("x = 0"),
            Point { x: _, y: 0 } => println!("y = 0"),
            Point { x, y } => println!("x and y are not equal to 0"),
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        let msg = Message::ChangeColor(Color::Rgb(10, 1, 161));

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change the color to hue {h}, saturation {s}, value {v}");
            }
        }

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 11, y: 20 });
    }
    pub fn ignoring_patterns_value() {
        let mut setting_value: Option<i32> = None;
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("seeting_value and new_setting_value cant owerwrite");
            }
            _ => setting_value = new_setting_value,
        }
        // println!("{:?} {:?}", setting_value, new_setting_value);

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        let numbers = (1, 2, 3, 4, 5);

        match numbers {
            (first, .., last) => {
                println!("first and last values in cortedge: {first}, {last}")
            }
            _ => (),
        };

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        let num = Some(15);

        if let Some(_x) = num {
            println!("x = {_x}");
        }
    }
    pub fn match_guards() {
        let five = Some(5);
        let for_cmp = 5;
        let no = false;

        match five {
            Some(x) if x == for_cmp => println!("x = {for_cmp}"),
            Some(4) | Some(5) | Some(6) if no => println!("number is 4, 5 or 6 and no = true"),

            Some(x) if x % 2 == 0 => println!("{x} is even"),
            Some(x) => println!("{x} not even"),

            _ => (),
        };

        // - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

        enum Numbers {
            a { anum: i32 },
            b { bnum: i32 },
        }

        let n = Numbers::a { anum: 3 };

        match n {
            Numbers::a { anum: abc @ 3..=7 } => println!("abc = {abc}"),
            _ => (),
        };
    }
}

fn main() {
    // 1. Base (learn_base_about_patterns module)
    //
    // matches();
    // let_expressions();

    // 2. Refutability (learn_refutability module)
    //
    // refutability();

    // 3. Syntax of patterns(learn_pattern_syntax module)
    //
    // multi_patterns();
    // ranges();
    // destructuring();
    // ignoring_patterns_value();
    // match_guards();
}
