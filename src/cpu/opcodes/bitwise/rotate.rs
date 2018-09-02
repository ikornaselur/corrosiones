use cpu::utils::add_byte_to_accumulator;
use cpu::{Addressing, CPU};

/// Rotate left
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
pub fn rol(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 6,
        Addressing::AbsoluteX => 7,
        Addressing::Accumulator => 2,
        Addressing::ZeroPage => 5,
        Addressing::ZeroPageX => 6,
        _ => panic!("ROL doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, false);
    let rotated = if cpu.flags.carry {
        byte << 1 | 1
    } else {
        byte << 1
    };
    cpu.write_byte(&addressing, rotated, true);

    cpu.flags.set_carry(byte >> 7 == 1);
    cpu.flags.set_zero_from_byte(rotated);
    cpu.flags.set_negative_from_byte(rotated);

    cycles
}

/// Rotate right
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
pub fn ror(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 6,
        Addressing::AbsoluteX => 7,
        Addressing::Accumulator => 2,
        Addressing::ZeroPage => 5,
        Addressing::ZeroPageX => 6,
        _ => panic!("ROR doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, false);
    let rotated = if cpu.flags.carry {
        byte >> 1 | 1 << 7
    } else {
        byte >> 1
    };
    cpu.write_byte(&addressing, rotated, true);

    cpu.flags.set_carry(byte & 1 == 1);
    cpu.flags.set_zero_from_byte(rotated);
    cpu.flags.set_negative_from_byte(rotated);

    cycles
}

/// Rotate right in memory, then AND accumulator with memory
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
pub fn rla(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX | Addressing::AbsoluteY => 7,
        Addressing::IndirectX | Addressing::IndirectY => 8,
        Addressing::ZeroPage => 5,
        _ => panic!("RLA doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, false);
    let rotated = if cpu.flags.carry {
        byte >> 1 | 1 << 7
    } else {
        byte >> 1
    };
    cpu.write_byte(&addressing, rotated, true);

    cpu.a &= rotated;

    cpu.flags.set_carry(byte & 1 == 1);
    cpu.flags.set_zero_from_byte(rotated);
    cpu.flags.set_negative_from_byte(rotated);

    cycles
}

/// Rotate right in memory, then add memory to accumulator with carry
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
/// * Overflow
pub fn rra(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX | Addressing::AbsoluteY => 7,
        Addressing::IndirectX | Addressing::IndirectY => 8,
        Addressing::ZeroPage => 5,
        _ => panic!("RRA doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, false);
    let rotated = if cpu.flags.carry {
        byte >> 1 | 1 << 7
    } else {
        byte >> 1
    };
    cpu.write_byte(&addressing, rotated, true);

    cpu.flags.set_carry(byte & 1 == 1);
    cpu.flags.set_zero_from_byte(rotated);
    cpu.flags.set_negative_from_byte(rotated);

    add_byte_to_accumulator(cpu, byte);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rol_rotates_with_carry() {
        let mut cpu = CPU {
            a: 0b1001_0110,
            ..CPU::default()
        };
        cpu.flags.set_carry(true);

        rol(&mut cpu, &Addressing::Accumulator);

        assert_eq!(cpu.a, 0b0010_1101);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn rol_rotates_in_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0101_0101, 0x01, 0x00])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        rol(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b1010_1011);
    }

    #[test]
    fn ror_rotates_into_carry() {
        let mut cpu = CPU {
            a: 0b1001_1001,
            ..CPU::default()
        };
        cpu.flags.set_carry(true);

        ror(&mut cpu, &Addressing::Accumulator);

        assert_eq!(cpu.a, 0b1100_1100);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn ror_rotates_in_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0101_0101, 0x01, 0x00])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        ror(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b1010_1010);
    }

    #[test]
    fn rla_rotates_in_memory_and_then_ands_with_accumulator() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0b1111_0000,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0101_0101, 0x01, 0x00])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        rla(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b1010_1010);
        assert_eq!(cpu.a, 0b1010_0000);
    }

    #[test]
    fn rra_rotates_in_memory_then_adds_memory_to_accumulator() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0b1111_0000,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0b0101_0101, 0x01, 0x00])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        rra(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0b1010_1010);
        assert_eq!(cpu.a, 70);
    }
}
