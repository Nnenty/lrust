use std::{thread, vec};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}
use ShirtColor::{Blue, Red};

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_prefer: Option<ShirtColor>) -> ShirtColor {
        user_prefer.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                Red => num_red += 1,
                Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            Red
        } else {
            Blue
        }
    }
}

fn run_giveaway() {
    let store = Inventory {
        shirts: vec![Red, Blue, Red],
    };

    let user_prefer = Some(Blue);
    let user_prefer2 = None;

    let giveaway1 = store.giveaway(user_prefer);

    let giveaway2 = store.giveaway(user_prefer2);

    println!("People with {:?} prefer gets {:?}", user_prefer, giveaway1);
    println!("People with {:?} prefer gets {:?}", user_prefer2, giveaway2);
}
fn borrows() {
    // only borrow
    {
        let list = vec![1, 2, 3];

        let borrow = || println!("print borrow {list:?}");

        borrow();
        println!("print list {list:?}");
        borrow();
    }
    // mutable borrow
    {
        let mut list = vec![1, 2, 3];

        println!("Before calling closure: {list:?}");

        let mut mut_borrow = || list.push(4);

        // println!("{list:?}");

        mut_borrow();

        println!("After calling closure: {list:?}");
    }
    // ownership transfer
    {
        let list = vec![1, 2, 3];

        thread::spawn(move || println!("from thread: {list:?}"))
            .join()
            .unwrap();

        // println("{list:?}");
    }
}
fn sort_by_key() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 13,
        },
        Rectangle {
            width: 1,
            height: 15,
        },
        Rectangle {
            width: 5,
            height: 30,
        },
    ];

    list.sort_by_key(|list| list.width);

    dbg!(list);
}
fn main() {
    // run_giveaway();
    // borrows();
    // sort_by_key();

    Shoe::run();
}
#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}
impl Shoe {
    fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    fn run() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let my_size_shoes = Shoe::shoe_in_size(shoes, 10);

        println!("{my_size_shoes:?}");
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn manually_iter() {
        let mut v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter_mut();

        assert_eq!(v1_iter.next(), Some(&mut 1));
        assert_eq!(v1_iter.next(), Some(&mut 2));
        assert_eq!(v1_iter.next(), Some(&mut 3));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iter_sum() {
        let vec = vec![1, 2, 3];

        let vec_iter = vec.iter();

        let total: i32 = vec_iter.sum();

        assert_eq!(total, 6);
    }
    #[test]
    fn iter_map() {
        let vec = vec![1, 2, 3];

        let result: Vec<_> = vec.iter().map(|x| x + 1).collect();

        assert_eq!(result, [2, 3, 4]);
    }
}
