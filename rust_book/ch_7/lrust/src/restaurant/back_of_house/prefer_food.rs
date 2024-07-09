pub enum Appetizer {
    Soup,
    Salad,
}
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Self {
        Self {
            toast: String::from(toast),
            seasonal_fruit: String::from("apples"),
        }
    }
}
