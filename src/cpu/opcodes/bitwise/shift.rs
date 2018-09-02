use cpu::{Addressing, CPU};

/// Arithmetic shift left
///
/// # Supported addressing modes
///
/// * Accumulator - 2 Cycles
/// * Absolute - 6 Cycles
/// * Absolute X - 7 Cycles
/// * Zero Page - 5 Cycles
/// * Zero Page X - 6 Cycles
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Zero
pub fn asl(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 6,
        Addressing::AbsoluteX => 7,
        Addressing::Accumulator => 2,
        Addressing::ZeroPage => 5,
        Addressing::ZeroPageX => 6,
        _ => panic!("ASL doesn't support {:?} addressing", addressing),
    };

    let old_byte = cpu.read_byte(&addressing, false);
    let byte = old_byte << 1;
    cpu.write_byte(&addressing, byte, true);

    cpu.flags.set_carry(old_byte >> 7 == 1);
    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);

    cycles
}

/// Arithmetic shift right
///
/// # Supported addressing modes
///
/// * Accumulator - 2 Cycles
/// * Absolute - 6 Cycles
/// * Absolute X - 7 Cycles
/// * Zero Page - 5 Cycles
/// * Zero Page X - 6 Cycles
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Zero
pub fn lsr(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 6,
        Addressing::AbsoluteX => 7,
        Addressing::Accumulator => 2,
        Addressing::ZeroPage => 5,
        Addressing::ZeroPageX => 6,
        _ => panic!("LSR doesn't support {:?} addressing", addressing),
    };

    let old_byte = cpu.read_byte(&addressing, false);
    let byte = old_byte >> 1;
    cpu.write_byte(&addressing, byte, true);

    cpu.flags.set_carry(old_byte & 1 == 1);
    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);

    cycles
}

/// Shift left one bit in memory, then OR accumulator with memory
///
/// *Undocumented instruction*
///
/// # Supported addressing modes
///
/// * Absolute - 6 Cycles
/// * Absolute X - 7 Cycles
/// * Absolute Y - 7 Cycles
/// * Indirect X - 8 Cycles
/// * Indirect Y - 8 Cycles
/// * Zero Page - 5 Cycles
/// * Zero Page X - 6 Cycles
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Zero
pub fn slo(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX | Addressing::AbsoluteY => 7,
        Addressing::IndirectX | Addressing::IndirectY => 8,
        Addressing::ZeroPage => 5,
        _ => panic!("SLO doesn't support {:?} addressing", addressing),
    };

    let old_byte = cpu.read_byte(&addressing, false);
    let byte = old_byte << 1;
    cpu.write_byte(&addressing, byte, true);

    cpu.a |= byte;

    cpu.flags.set_carry(old_byte >> 7 == 1);
    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);

    cycles
}

/// Shift right one bit in memory, then EOR accumulator with memory
///
/// *Undocumented instruction*
///
/// # Supported addressing modes
///
/// * Absolute - 6 Cycles
/// * Absolute X - 7 Cycles
/// * Absolute Y - 7 Cycles
/// * Indirect X - 8 Cycles
/// * Indirect Y - 8 Cycles
/// * Zero Page - 5 Cycles
/// * Zero Page X - 6 Cycles
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Zero
pub fn sre(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX | Addressing::AbsoluteY => 7,
        Addressing::IndirectX | Addressing::IndirectY => 8,
        Addressing::ZeroPage => 5,
        _ => panic!("SLO doesn't support {:?} addressing", addressing),
    };

    let old_byte = cpu.read_byte(&addressing, false);
    let byte = old_byte >> 1;
    cpu.write_byte(&addressing, byte, true);

    cpu.a ^= byte;

    cpu.flags.set_carry(old_byte >> 7 == 1);
    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn asl_shifts_into_carry() {
        let mut cpu = CPU {
            a: 0b1001_0110,
            ..CPU::default()
        };

        asl(&mut cpu, &Addressing::Accumulator);

        assert_eq!(cpu.a, 0b0010_1100);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn asl_shifts_in_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0101_0101, 0x01, 0x00])
            .expect("Failed to load ram");

        asl(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b1010_1010);
    }

    #[test]
    fn lsr_shifts_into_carry() {
        let mut cpu = CPU {
            a: 0b1001_1001,
            ..CPU::default()
        };

        lsr(&mut cpu, &Addressing::Accumulator);

        assert_eq!(cpu.a, 0b0100_1100);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn lsr_shifts_in_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0101_0101, 0x01, 0x00])
            .expect("Failed to load ram");

        lsr(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b0010_1010);
    }

    #[test]
    fn lsr_sets_zero_flag() {
        let mut cpu = CPU {
            a: 0x01,
            ..CPU::default()
        };

        lsr(&mut cpu, &Addressing::Accumulator);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn slo_shifts_left_in_memory_and_then_ors_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1010,
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0000_1010, 0x01, 0x00])
            .expect("Failed to load ram");

        slo(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b0001_0100);
        assert_eq!(cpu.a, 0b0101_1110);
    }

    #[test]
    fn sre_shifts_right_in_memory_and_then_eors_with_accumulator() {
        let mut cpu = CPU {
            a: 0b0101_1111,
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0000_1010, 0x01, 0x00])
            .expect("Failed to load ram");

        sre(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b0000_0101);
        assert_eq!(cpu.a, 0b0101_1010);
    }
}
