use cpu::CPU;
/// Set the overflow bit based on the two bytes provided
///
/// Formula from http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
fn get_overflow(m: u8, n: u8, result: u8) -> bool {
    (m ^ result) & (n ^ result) & 0x80 != 0
}

pub fn add_byte_to_accumulator(cpu: &mut CPU, original_byte: u8) {
    let (byte, byte_carry) = if cpu.flags.carry {
        original_byte.overflowing_add(1)
    } else {
        (original_byte, false)
    };

    let (result, carry) = cpu.a.overflowing_add(byte);

    let overflow = get_overflow(original_byte, cpu.a, result);
    cpu.flags.set_overflow(overflow);

    cpu.a = result;
    cpu.flags.set_carry(carry || byte_carry);
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
}
