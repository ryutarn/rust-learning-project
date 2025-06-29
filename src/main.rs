fn main() {
    println!("{}", get_hello_message());
}

fn get_hello_message() -> String {
    String::from("Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hello_message() {
        assert_eq!(get_hello_message(), "Hello, world!");
    }
}
