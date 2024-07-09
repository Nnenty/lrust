use learn_boxes_and_deref::{hello, MyBox, RecursList};
mod learn_boxes_and_deref {
    use std::ops::Deref;

    #[derive(Debug)]
    pub enum RecursList {
        Cons(i32, Box<RecursList>),
        Nil,
    }
    use RecursList::{Cons, Nil};
    impl RecursList {
        pub fn test_recurslist() {
            let recurslist = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

            println!("{recurslist:?}");
        }
    }

    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> Self {
            Self(x)
        }
        pub fn mybox_test() {
            let x = 6.1;
            let y = MyBox::new(x);

            println!("{x} assert eq {}", *y);
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    pub fn hello(name: &str) {
        println!("Hello, {name}")
    }
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn deref() {
            let x = 3;
            let y = &x;

            assert_eq!(3, *y);
        }
        #[test]
        fn box_deref() {
            let x = 3;
            let y = Box::new(x);

            assert_eq!(3, *y);
        }
    }
}

use learn_drop::CustomSmartPointer;
mod learn_drop {
    #[derive(Debug)]
    pub struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("{self:?} was dropped");
        }
    }
    impl CustomSmartPointer {
        pub fn new(data: String) -> Self {
            Self { data: data }
        }
        pub fn test_drop() {
            let data = CustomSmartPointer::new("hello world".to_string());
            let data2 = CustomSmartPointer::new("hello rust".to_string());

            println!("data and data2 variables was created");

            drop(data2);
            println!("data2 was dropped before the end of main!");
        }
    }
}

use learn_Rc::List;
mod learn_Rc {
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    impl List {
        pub fn test_list() {
            let a = Rc::new(Cons(3, Rc::new(Nil)));
            println!("rc count after creating a: {}", Rc::strong_count(&a));

            let b = Rc::new(Cons(4, Rc::clone(&a)));
            println!("rc count after creating b: {}", Rc::strong_count(&a));

            {
                let c = Rc::new(Cons(5, Rc::new(Cons(6, Rc::clone(&a)))));

                println!("rc count after creating c: {}", Rc::strong_count(&a));
                // println!("\nc = {c:?}\n");
            }
            println!(
                "rc count after c goes out from the scope: {}",
                Rc::strong_count(&a)
            );

            // println!("\na = {a:?}\nb = {b:?}");
        }
    }
}

use learn_RefCell::{LimitTracker, Messenger, MockMessenger};
mod learn_RefCell {
    use std::cell::RefCell;

    pub trait Messenger {
        fn send(&self, msg: &str);
    }
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> Self {
            Self {
                messenger: messenger,
                max: max,
                value: 0,
            }
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Warning: You've used up over 90% of your quota!")
            } else if percentage_of_max >= 0.7 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!")
            }
        }
        pub fn test_limit_tracker() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(87);
            limit_tracker.set_value(95);
            limit_tracker.set_value(50);

            dbg!(mock_messenger);
        }
    }
    #[derive(Debug)]
    pub struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        pub fn new() -> Self {
            Self {
                sent_message: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_message.borrow_mut().push(String::from(msg))
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_message() {
            let mock = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock, 100);

            limit_tracker.set_value(66);
            limit_tracker.set_value(90);

            assert_eq!(mock.sent_message.borrow().len(), 1);
            assert_eq!(
                mock.sent_message.borrow()[0],
                "Warning: You've used up over 90% of your quota!"
            );
        }
    }
}

use learn_RefCell_with_Rc::test_mut_rc;
mod learn_RefCell_with_Rc {
    use std::cell::RefCell;
    use std::rc::Rc;
    use List::{Cons, Nil};

    #[derive(Debug)]
    pub enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    pub fn test_mut_rc() {
        let value = Rc::new(RefCell::new(1));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Rc::new(Cons(Rc::new(RefCell::new(2)), Rc::clone(&a)));
        let c = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));

        *value.borrow_mut() += 10;

        println!("a = {a:?}");
        println!("b = {b:?}");
        println!("c = {c:?}");
    }
}

use learn_ref_cycles::List as CycleList;
mod learn_ref_cycles {
    use std::rc::Rc;
    use std::{borrow::Borrow, cell::RefCell};
    use List::{Cons, Nil};

    #[derive(Debug)]
    pub enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, r) => Some(r),
                Nil => None,
            }
        }
        pub fn test_ref_cycle() {
            let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));

            println!("Initial a strong count = {}", Rc::strong_count(&a));
            println!("Next item = {:?}", a.tail());

            let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));

            println!("Initial b strong count = {}", Rc::strong_count(&b));
            println!("Next item = {:?}", b.tail());

            if let Some(link) = a.tail() {
                *link.borrow_mut() = Rc::clone(&b);
            }

            println!("a strong count: {}", Rc::strong_count(&a));
            println!("b string count: {}", Rc::strong_count(&b));

            // println!("Next item: {:?}", a.tail());
        }
    }
}

fn main() {
    // 1.Box<T> (learn_boxes_and_deref module)
    //
    // RecursList::test_recurslist();
    // MyBox::<u8>::mybox_test();
    //
    // let str = &String::from("rust");
    // hello(&&&&&&&str);

    // 2.Drop (learn_drop module)
    //
    // CustomSmartPointer::test_drop();

    // 3.Rc<T> (learn_Rc module)
    //
    // List::test_list();

    // 4.RefCell<T> (learn_RefCell module)
    //
    // LimitTracker::<MockMessenger>::test_limit_tracker();

    // 5.RefCell<T> with Rc<T> (learn_RefCell_with_Rc module)
    //
    // test_mut_rc();

    // 6.Reference cycles (learn_ref_cycles module)
    //
    // CycleList::test_ref_cycle();
}
