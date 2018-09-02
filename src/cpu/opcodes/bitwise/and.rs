use cpu::{Addressing, CPU};

/// And memory with accumulator
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
/// * Negative
/// * Zero
pub fn and(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute
        | Addressing::AbsoluteX
        | Addressing::AbsoluteY
        | Addressing::ZeroPageX => 4,
        Addressing::Immediate => 2,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        _ => panic!("AND doesn't support {:?} addressing", addressing),
    };

    cpu.a &= cpu.read_byte(&addressing, true);
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
    cycles
}

/// And memory with accumulator and set carry if negative
///
/// *Undocumented instruction*
///
/// # Supported addressing modes
///
/// * Immediate - 2 Cycles
///
/// # Flags affected
///
/// * Negative
/// * Zero
/// * Carry
pub fn aac(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Immediate => 2,
        _ => panic!("ACC doesn't support {:?} addressing", addressing),
    };

    cpu.a &= cpu.read_byte(&addressing, true);

    if cpu.a & (1 << 7) > 0 {
        cpu.flags.set_carry(true);
    }
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
    cycles
}

/// And memory with accumulator, then shift right one bit
///
/// *Undocumented instruction*
///
/// # Supported addressing modes
///
/// * Immediate - 2 cycles
///
/// # Flags affected
///
/// * Negative
/// * Zero
/// Carry
pub fn asr(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Immediate => 2,
        _ => panic!("ASR doesn't support {:?} addressing", addressing),
    };

    cpu.a &= cpu.read_byte(&addressing, true);

    cpu.flags.set_carry(cpu.a & 1 == 1);

    cpu.a = cpu.a >> 1;
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative(false);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn and_ands_accumulator_with_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0b1111_0000,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0b1010_1010, 0xFF])
            .expect("Failed to load ram");

        and(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b1010_0000);
    }

    #[test]
    fn aac_ands_memory_with_accumulator_and_sets_carry_if_negative_bit_is_set() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0b1111_1111,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0b1000_0000])
            .expect("Failed to load ram");

        aac(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b1000_0000);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn asr_ands_memory_with_accumulator_and_shifts_one_to_the_right() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0b1111_1111,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0b1010_0101])
            .expect("Failed to load ram");
        cpu.flags.set_zero(true);
        cpu.flags.set_negative(true);

        asr(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b0101_0010);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, false);
        assert_eq!(cpu.flags.negative, false);
    }
}
