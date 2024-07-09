use learn_unsafe_rust::{
    global_var, initialize_rav_pointers, test_own_split_at_mut, unsafe_func_init,
};
mod learn_unsafe_rust {
    pub fn initialize_rav_pointers() {
        let mut num = 5;
        let pointer3: *const String;

        {
            let s = "hello".to_string();
            pointer3 = &s as *const String;
        }

        let pointer1 = &mut num as *mut i32;
        let pointer2 = &mut num as *mut i32;

        unsafe {
            *pointer1 += 1;
            println!("{}", *pointer1);

            *pointer2 += 100000;
            println!("{}, {}, {}", *pointer2, *pointer1, *pointer3);
        }
    }

    pub fn unsafe_func_init() {
        unsafe fn func() {}

        // without unsafe block program not compile
        unsafe {
            func();
        }
    }

    pub fn unsafe_trait_init() {
        unsafe trait Foo {
            fn bar() {}
        }

        unsafe impl Foo for i32 {}
        i32::bar();
    }

    pub fn test_own_split_at_mut() {
        use std::slice;

        fn own_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let ptr = values.as_mut_ptr();
            let len = values.len();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        let mut nums = vec![1, 2, 3, 4, 5, 6];

        let (firsth, secondh) = own_split_at_mut(&mut nums, 3);

        println!("first half of vector: {:?}, second: {:?}", firsth, secondh);
    }

    static mut GLOBAL: i32 = 0;
    const ONE: i32 = 1;

    pub fn global_var() {
        fn use_global() {
            unsafe {
                GLOBAL += ONE;
            }
        }

        use_global();
        use_global();

        unsafe {
            println!("global var = {GLOBAL}");
        }
    }
}

use learn_advanced_traits::{
    equal_associated_functions_names, equal_methods_names, newtype_pattern, supertrait,
    use_add_trait,
};
mod learn_advanced_traits {
    use std::path::Display;

    pub fn use_add_trait() {
        use std::ops::Add;

        // what does the trait 'Add' look from the inside:
        //
        // trait Add<Rhs = Self> {
        // type Output;
        //
        // fn add(self, rhs: Rhs) -> Self::Output;
        //}

        #[derive(Debug)]

        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, other: Meters) -> Millimeters {
                Millimeters(self.0 + (other.0 * 1000))
            }
        }

        let val = Millimeters(13);
        let val_meters = Meters(1);

        println!("val + val_meters = {:?}", val.add(val_meters));
    }

    pub fn equal_methods_names() {
        trait Bird {
            fn fly(&self);
        }
        trait AirPlane {
            fn fly(&self);
        }

        struct Human {}

        impl Bird for Human {
            fn fly(&self) {
                println!("fly with wings");
            }
        }
        impl AirPlane for Human {
            fn fly(&self) {
                println!("fly with engine");
            }
        }
        impl Human {
            fn fly(&self) {
                println!("can't fly");
            }
        }

        let person = Human {};

        Bird::fly(&person);
        AirPlane::fly(&person);

        person.fly();
        // or Human::fly(&person);
    }

    pub fn equal_associated_functions_names() {
        struct Dog;

        trait Animal {
            fn baby_dog() -> String;
        }

        impl Animal for Dog {
            fn baby_dog() -> String {
                "Spot".to_string()
            }
        }
        impl Dog {
            fn baby_dog() -> String {
                "Puppy".to_string()
            }
        }

        println!("puppy: {}", Dog::baby_dog());
        println!("spot: {}", <Dog as Animal>::baby_dog());
    }

    pub fn supertrait() {
        trait OutlinePrint: std::fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();

                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        impl std::fmt::Display for Point {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}, {}", self.x, self.y)
            }
        }
        impl OutlinePrint for Point {}

        let point = Point { x: 2, y: 3 };
        point.outline_print();
    }

    pub fn newtype_pattern() {
        struct Wrapper(Vec<String>);

        impl std::fmt::Display for Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
    }
}

use learn_advanced_types::{never_type, short_long_types, type_synonyms};
mod learn_advanced_types {
    pub fn type_synonyms() {
        fn func_for_int(num: i32) {
            println!("this function can get OtherInt, because type OtherInt is equal i32");
        }

        type OtherInt = i32;

        let x: OtherInt = 1;
        let y: i32 = x;

        println!("x + y = {}", x + y);

        func_for_int(x);
    }

    pub fn short_long_types() {
        type AnyType = Box<dyn Fn() + Send + 'static>;

        let var: AnyType = Box::new(|| println!("hello"));

        fn takes_long_type(v: AnyType) {}

        fn return_long_type() -> AnyType {
            Box::new(|| println!(""))
        }

        type Result<T> = std::result::Result<T, String>;

        fn return_ok() -> Result<()> {
            Ok(())
        }

        fn return_err() -> Result<()> {
            Err("".to_string())
        }
    }

    pub fn never_type() {
        let num = Some(1);

        match num {
            // return i32
            Some(v) => v,

            // return !(never)
            None => panic!("variable is none"),
        };
    }
}

use learn_advsnced_functions::func_pointers;
mod learn_advsnced_functions {
    pub fn func_pointers() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        fn use_func_twice(f: fn(i32) -> i32) {
            println!("result: {}", f(1) + f(1));
        }

        use_func_twice(add_one);
        use_func_twice((|n| n));

        fn return_closure() -> Box<dyn Fn()> {
            Box::new((|| println!("")))
        }
    }
}

use learn_macros::{create_own_macro, test_derive};
mod learn_macros {
    pub fn create_own_macro() {
        #[macro_export]
        macro_rules! vec2 {
            ($ ($x: expr), *) => {
            {
                let mut temp_vec = Vec::new();

                $(
                    temp_vec.push($x);
                )*

                temp_vec
                }
            };
        }

        println!("{:?}", vec2![1, 2, 3]);
    }

    pub fn test_derive() {
        use hello_macro::HelloMacro;
        use hello_macro_derive::HelloMacro;

        #[derive(HelloMacro)]
        struct Struct;

        Struct::hello_macro();
    }
}

fn main() {
    // 1. Unsafe Rust code (learn_unsafe_rust module)
    //
    // initialize_rav_pointers();
    // unsafe_func_init();
    // unsafe_trait_init();
    //
    // test_own_split_at_mut();
    // global_var();

    // 2. Advanced traits (learn_advanced_traits module)
    //
    // use_add_trait();
    // equal_methods_names();
    // equal_associated_functions_names();
    // supertrait();
    // newtype_pattern();

    // 3. Advanced types(learn_advanced_types module)
    //
    // type_synonyms();
    // short_long_types();
    // never_type();

    // 4. Advanced functions(learn_advsnced_functions module)
    //
    // func_pointers();

    // 5. Macros (learn_macros module)
    //
    // create_own_macro();
    // test_derive();
}
