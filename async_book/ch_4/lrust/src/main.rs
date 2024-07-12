use may_problem_without_pin::Test;
use pin_to_heap::TestHeapPin;
use pin_to_stack::TestPin;

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
mod pin_to_stack {
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    pub struct TestPin {
        a: String,
        a_ptr: *const String,

        _marker: PhantomPinned,
    }

    impl TestPin {
        pub fn new(text: &str) -> Self {
            TestPin {
                a: text.to_string(),
                a_ptr: std::ptr::null(),

                _marker: PhantomPinned,
            }
        }
        pub fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };

            this.a_ptr = self_ptr;
        }

        pub fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }
        pub fn a_ptr(self: Pin<&Self>) -> &String {
            assert!(
                !self.a_ptr.is_null(),
                "Test::b called without Test::init being called first"
            );

            unsafe { &*(self.a_ptr) }
        }
        pub fn use_testpin() {
            let mut test1 = TestPin::new("testpin");
            // shadind required
            let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
            test1.as_mut().init();

            let mut test2 = TestPin::new("second testpin");
            let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
            test2.as_mut().init();

            println!("TESTPIN1: {}, {}", test1.a, unsafe { &*(test1.a_ptr) });

            // if you uncomment line under you will see a error of compilation:
            // std::mem::swap(test1.get_mut(), test2.get_mut());

            println!("TESTPIN2: {}, {}", test2.a, TestPin::a_ptr(test2.as_ref()));
        }
    }
}

mod pin_to_heap {
    use std::future::Future;
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    pub struct TestHeapPin {
        a: String,
        a_ptr: *const String,
        _marker: PhantomPinned,
    }

    impl TestHeapPin {
        pub fn new(text: &str) -> Pin<Box<Self>> {
            let test = TestHeapPin {
                a: text.to_string(),
                a_ptr: std::ptr::null(),

                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(test);

            let self_ptr = &boxed.a;

            unsafe { boxed.as_mut().get_unchecked_mut().a_ptr = self_ptr };

            boxed
        }
        pub fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }
        pub fn a_ptr(self: Pin<&Self>) -> &String {
            unsafe { &*(self.a_ptr) }
        }
        pub fn use_testheappin() {
            let mut test1 = TestHeapPin::new("testheappin");
            let mut test2 = TestHeapPin::new("second testheappin");

            println!("TestHeapPin: {}, {}", test1.a, unsafe { &*(test1.a_ptr) });

            println!(
                "TestHeapPin2: {}, {}",
                test2.a,
                TestHeapPin::a_ptr(test2.as_ref())
            );
        }
    }

    pub fn execute_unpin_future(f: impl Future<Output = ()> + Unpin) {
        // expect unpin feature function
    }

    // if you want use func that expect Unpin future you can
    // pass Pin<Box<T>>
    pub async fn test_exunf() {
        let fut = async {};
        let fut = Box::pin(fut);

        // it works
        execute_unpin_future(fut);
    }
}

fn main() {
    // Modules in this programm:
    //
    // Modules with funcs to use:
    // 1. mod may_problem_without_pin
    // 2. mod pin_to_stack
    // 3. mod pin_to_heap

    Test::use_test();
    TestPin::use_testpin();
    TestHeapPin::use_testheappin();
}
