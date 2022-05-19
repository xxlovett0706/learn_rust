pub fn hello_rust() -> String {
    "Hello Rust!".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn equal_hello_rust() {
        assert_eq!(crate::hello_rust(), String::from("Hello Rust!"));
    }
}
