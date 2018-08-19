/// Set the overflow bit based on the two bytes provided
///
/// Formula from http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
pub fn get_overflow(m: u8, n: u8, result: u8) -> bool {
    (m ^ result) & (n ^ result) & 0x80 != 0
}
