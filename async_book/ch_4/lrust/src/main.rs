use may_problem_without_pin::Test;
mod may_problem_without_pin {
    #[derive(Debug)]
    pub struct Test {
        a: String,
        a_ptr: *const String,
    }

    impl Test {
        pub fn new(text: &str) -> Self {
            Test {
                a: text.to_string(),
                a_ptr: std::ptr::null(),
            }
        }
        pub fn init(&mut self) {
            let self_ref: *const String = &self.a;

            self.a_ptr = self_ref;
        }

        pub fn a(&self) -> &str {
            &self.a
        }
        pub fn a_ptr(&self) -> &String {
            assert!(
                !self.a_ptr.is_null(),
                "Test::b called without Test::init being called first"
            );

            unsafe { &*(self.a_ptr) }
        }
        pub fn use_test() {
            let mut test = Test::new("hello");
            test.init();

            let mut test2 = Test::new("second hello");
            test2.init();

            println!("test1: {}, {}", test.a(), test.a_ptr());

            // test2.a is points to test.a, but test2.a_ptr still
            // points at his old data (test2.a before mem::swap).
            // its print: test2: hello, second hello
            //   ('hello because swap') ('second hello' because
            //                              pointer to old data)
            std::mem::swap(&mut test, &mut test2);
            println!("test2: {}, {}", test2.a(), test2.a_ptr());
        }
    }
}
mod pin_to_stack {}

fn main() {
    // Modules in this programm:
    //
    // Modules with funcs to use:
    // 1. mod may_problem_without_pin
    // 2. mod pin_to_stack
    Test::use_test();
}
