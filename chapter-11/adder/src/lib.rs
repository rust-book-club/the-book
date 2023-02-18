pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        // not actual, expected
        // not expected, actual
        // just left and right
        assert_eq!(4, add_two(3), "{} should equal {}", 4, add_two(3));
    }
}
