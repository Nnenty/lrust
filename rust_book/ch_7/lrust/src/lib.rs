pub mod restaurant;

use restaurant::{back_of_house, front_of_house};

use back_of_house::prefer_food::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    let food = Appetizer::Soup;

    match food {
        Appetizer::Salad => println!("I want eat salad"),
        Appetizer::Soup => println!("I want eat soup"),
    };

    println!("with {} toast, please..", meal.toast);

    meal.toast = String::from("Wheat");

    println!("Or {} toast, please", meal.toast);
}
