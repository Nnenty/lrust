use lrust::add_two;

mod module;

// to use only this test use cargo test --test integration_test
#[test]
fn test_lib() {
    let result = add_two(2);

    println!("The result is {result}");

    module::module_func();

    assert_eq!(4, result);
}
