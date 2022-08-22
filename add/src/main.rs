// #![cfg_attr(coverage_nightly, feature(no_coverage))] // Doesn't work yet
#![feature(no_coverage)]

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_coverage]
fn main() {
    println!("add: 1 + 2 = {}", add(1, 2));
}

#[cfg(test)]
mod test {
    use super::*;

    //#[cfg_attr(coverage_nightly, no_coverage)] // Doesn't work yet?
    #[no_coverage]
    #[test]
    fn test_succeeds() {
        assert_eq!(add(1, 1), 2);
    }

    #[no_coverage]
    #[test]
    fn test_negatives() {
        assert_eq!(add(-1, -2), -3);
    }

    // To run this test:
    //  `cargo xt test -- --include-ignored` or
    //  `cargo test -- --include-ignored`
    #[no_coverage]
    #[test]
    fn test_fails() {
        assert_ne!(add(1, 1), 1);
    }
}
