pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn divide() {
        let zero = "".len();
        1 / zero;
    }
}
