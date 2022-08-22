//#![cfg_attr(coverage_nightly, feature(no_coverage))]
#![feature(no_coverage)]

fn sub(a: i32, b: i32) -> i32 {
    a + b
}

#[no_coverage]
fn main() {
    println!("sub: 1 - 2 = {}", sub(1, 2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[no_coverage]
    #[test]
    fn test_succeeds() {
        assert_eq!(sub(1, 1), 2);
    }

    #[no_coverage]
    #[test]
    fn test_negatives() {
        assert_eq!(sub(-1, -2), -3);
    }

    // To run this test:
    //  `cargo xt test -- --include-ignored` or
    //  `cargo test -- --include-ignored`
    #[no_coverage]
    #[test]
    fn test_fails() {
        assert_ne!(sub(1, 1), 1);
    }
}
