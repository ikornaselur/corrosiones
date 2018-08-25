use cpu::{Addressing, CPU};

/// Bit Test
///
/// This instructions is used to test if one or more bits are set in a target memory location. The
/// mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the
/// result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V flags.
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Zero Page - 3 Cycles
///
/// # Flags affected
///
/// * Negative
/// * Overflow
/// * Zero
pub fn bit(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 4,
        Addressing::ZeroPage => 3,
        _ => panic!("BIT doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    let overflow = byte & 1 << 6 > 0;
    cpu.flags.set_overflow(overflow);
    cpu.flags.set_negative_from_byte(byte);

    cpu.flags.set_zero_from_byte(cpu.a & byte);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_uses_mask_for_zero() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0b0000_1111,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0b0000_1111, 0x01, 0x00]);

        bit(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.flags.zero, false);

        cpu.a = 0b1111_0000;
        cpu.pc = 0x0002;

        bit(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn bit_copies_overflow_from_bit_6_to_flags() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0b0000_0000,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0b0100_0000, 0x01, 0x00]);

        bit(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.flags.overflow, true);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn bit_copies_negative_from_bit_7_to_flags() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0b0000_0000,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0b1000_0000, 0x01, 0x00]);

        bit(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.flags.negative, true);
        assert_eq!(cpu.flags.overflow, false);
    }
}
