pub mod memory;

pub fn foo() -> u8 {
    10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(foo(), 10);
    }
}
