struct Rectange {
    width: u32,
    height: u32,
}
impl Rectange {
    fn smaller_then(&self, other_rect: &Rectange) -> bool {
        self.width < other_rect.width && self.height < other_rect.height
    }
    fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

pub fn add_two(left: i32) -> i32 {
    left + 2
}
pub fn add_name(name: &str) -> String {
    format!("Hello {name}")
}
pub fn print_and_return(num: i32) -> i32 {
    println!("I got number {num}");

    num
}
fn private_func() {
    println!("This is private func");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private() {
        private_func();
    }

    #[test]
    fn can_hold() {
        let larger = Rectange::new(10, 5);
        let smaller = Rectange::new(5, 3);

        assert!(smaller.smaller_then(&larger));
    }

    #[test]
    #[should_panic]
    fn i_hate_name() {
        let name = "name";
        let text = add_name(name);

        assert!(
            !text.contains(name),
            "i dont like peoples whos name '{name}'"
        );
    }

    #[test]
    #[should_panic(expected = "from 1 to 100; not 200")]
    fn bad_range() {
        let num = 200;

        if num > 100 || num < 1 {
            panic!("Num should be in range from 1 to 100; not {num}");
        }
    }

    #[test]
    fn pass() {
        let num = print_and_return(10); // to show output use cargo test -- --show-output

        assert_eq!(num, 10);
    }

    // #[test]
    // fn err() {
    //     let num = print_and_return(7);

    //     assert_eq!(num, 5);
    // }

    #[test]
    #[ignore]
    fn ignore_test() {
        println!("This test should be ignored");
    }
}
