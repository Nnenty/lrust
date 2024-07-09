#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut rect1 = Rectangle::new();
    rect1 = Rectangle::from(20, 30);

    let rect2 = Rectangle::from(10, 20);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("The area of rect1 = {}", rect1.area());

    dbg!(&rect1);
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
    fn from(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }
    }
}
