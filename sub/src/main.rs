fn sub(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("sub: 1 - 2 = {}", sub(1, 2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_succeeds() {
        assert_eq!(sub(1, 1), 2);
    }

    #[test]
    fn test_negatives() {
        assert_eq!(sub(-1, -2), -3);
    }

    // To run this test:
    //  `cargo xt test -- --include-ignored` or
    //  `cargo test -- --include-ignored`
    #[ignore]
    #[test]
    fn test_fails() {
        assert_eq!(sub(1, 1), 1);
    }
}
