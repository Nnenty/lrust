use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
pub struct DynTypes<T, T2> {
    var1: T,
    var2: T2,
}
pub struct Message {
    from: String,
    to: String,
    message: String,
}
pub trait DisplayFuncs {
    fn debug_print(&self)
    where
        Self: Debug,
    {
        println!("{self:?}");
    }
    fn print(&self);
}

impl DisplayFuncs for Message {
    fn print(&self) {
        println!("\tFrom {} to {}:\n{}\n", self.from, self.to, self.message);
    }
}
impl<T, T2> DisplayFuncs for DynTypes<T, T2>
where
    T: Display,
    T2: Display,
{
    fn print(&self) {
        println!("var1 = {}, var2 = {}", self.var1, self.var2);
    }
}

impl<T, T2> DynTypes<T, T2> {
    fn new(a: T, b: T2) -> Self {
        Self { var1: a, var2: b }
    }
    fn mixup<T3, T4>(self, other: DynTypes<T3, T4>) -> DynTypes<T, T4> {
        DynTypes {
            var1: self.var1,
            var2: other.var2,
        }
    }
}
impl Message {
    fn new(from: String, to: String, message: String) -> Self {
        Self {
            from: from,
            to: to,
            message: message,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn print_largestfn() {
    let int = vec![3, 2, 7, 4, 1, 9, 10];
    let chars = vec!['a', 'b', 'c', 'd', 't', 'f', 'g'];
    let words = vec!["abc", "def", "ghi"];

    println!("Largest in integer list - {}", largest(&int));

    println!("Largest in chars list - {}", largest(&chars));

    println!("Largest in word list - {}", largest(&words));
}

fn main() {
    let t: &'static str = "i can live in any part of programm";

    let test = Message::new(
        "Belyashik".to_string(),
        "Maxik".to_string(),
        "Dear Maxik, I want to eat!".to_string(),
    );
    test.print();

    let test = DynTypes::new(10, 20.0);
    let for_mix = DynTypes::new('c', "Hello");

    let result = test.mixup(for_mix);

    result.print();

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {result}");
    }
}
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
fn first_rule(a: & /*'a*/ str, b: & /*'b*/ str, c: & /*'c*/ str) {
    // all variables get their own lifetime
}
fn second_rule(a: &str) -> &str {
    a
}
struct Test<'a> {
    text: &'a str,
}
impl<'a> Test<'a> {
    fn third_rule(&self, a: &str) -> &str {
        a;

        self.text
    }
}
