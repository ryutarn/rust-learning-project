use rust_learning_project::get_hello_message;

#[test]
fn test_get_hello_message() {
    assert_eq!(get_hello_message(), "Hello, world!");
}