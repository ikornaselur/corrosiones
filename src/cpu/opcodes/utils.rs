/// Set the overflow bit based on the two bytes provided
///
/// Formula from http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
pub fn get_overflow(m: u8, n: u8, result: u8) -> bool {
    (m ^ result) & (n ^ result) & 0x80 != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn overflow_no_unsigned_carry_or_signed_overflow() {
        let overflow = get_overflow(0x50, 0x10, 0x60);
        assert_eq!(overflow, false);

        let overflow = get_overflow(0x50, 0x90, 0xe0);
        assert_eq!(overflow, false);

        let overflow = get_overflow(0xD0, 0x10, 0xe0);
        assert_eq!(overflow, false);
    }

    #[test]
    fn overflow_no_unsigned_carry_but_signed_overflow() {
        let overflow = get_overflow(0x50, 0x50, 0xA0);
        assert_eq!(overflow, true);
    }

    #[test]
    fn overflow_unsigned_carry_but_no_signed_overflow() {
        let overflow = get_overflow(0x50, 0xD0, 0x20);
        assert_eq!(overflow, false);

        let overflow = get_overflow(0xD0, 0xD0, 0xA0);
        assert_eq!(overflow, false);
    }

    #[test]
    fn overflow_unsigned_carry_and_signed_overflow() {
        let overflow = get_overflow(0xD0, 0x90, 0x60);
        assert_eq!(overflow, true);
    }
}
