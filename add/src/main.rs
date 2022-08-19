fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("add: 1 + 2 = {}", add(1, 2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_succeeds() {
        assert_eq!(add(1, 1), 2);
    }

    // To run this test:
    //  `cargo xt test -- --include-ignored` or
    //  `cargo test -- --include-ignored`
    #[ignore]
    #[test]
    fn test_fails() {
        assert_eq!(add(1, 1), 1);
    }
}
