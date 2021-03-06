use cpu::opcodes::math::add::add_byte_to_accumulator;
use cpu::{Addressing, CPU};

/// Subtract with borrow
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Absolute X - 4* Cycles
/// * Absolute Y - 4* Cycles
/// * Immediate - 2 cycles
/// * Indirect X - 6 Cycles
/// * Indirect Y - 5* Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page X - 4 Cycles
///
/// \* Add 1 if page boundary is crossed
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Overflow
/// * Zero
pub fn sbc(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute
        | Addressing::AbsoluteX
        | Addressing::AbsoluteY
        | Addressing::ZeroPageX => 4,
        Addressing::Immediate => 2,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        _ => panic!("SBC doesn't support {:?} addressing", addressing),
    };

    let byte = !cpu.read_byte(&addressing, true);
    add_byte_to_accumulator(cpu, byte);

    cycles
}

/// Subtract one from memory without borrow
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
pub fn dcp(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::ZeroPage => 5,
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX | Addressing::AbsoluteY => 7,
        Addressing::IndirectX | Addressing::IndirectY => 8,
        _ => panic!("DCP doesn't support {:?} addressing", addressing),
    };

    let byte = !cpu.read_byte(&addressing, true);
    add_byte_to_accumulator(cpu, byte);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sbc_immediate_without_borrow() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 4,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x01, 0xFF, 0xFF])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true); // Carry == No borrow

        let cycles = sbc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 3);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbc_immediate_with_borrow() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 4,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x01, 0xFF, 0xFF])
            .expect("Failed to load ram");

        let cycles = sbc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sbc_immediate_with_borrow_and_max_value() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0xFF,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xAA, 0xFF, 0xAA, 0xAA])
            .expect("Failed to load ram");
        cpu.flags.set_carry(false);

        let cycles = sbc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn sbc_immediate_with_borrow_and_max_value_and_min_accumulator() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0x00,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xAA, 0xFF, 0xAA, 0xAA])
            .expect("Failed to load ram");
        cpu.flags.set_carry(false);

        let cycles = sbc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn sbc_immediate_without_borrow_and_max_value_and_min_accumulator() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0x00,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xAA, 0xFF, 0xAA, 0xAA])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        let cycles = sbc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.flags.carry, false);
    }
}
