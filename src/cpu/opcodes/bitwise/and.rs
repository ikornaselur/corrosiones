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

    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);

    let negative = cpu.flags.negative;
    cpu.flags.set_carry(negative);
    cycles
}

/// And memory with accumulator, then shift right one bit
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
pub fn asr(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Immediate => 2,
        _ => panic!("ASR doesn't support {:?} addressing", addressing),
    };

    cpu.a &= cpu.read_byte(&addressing, true);

    cpu.flags.set_carry(cpu.a & 1 == 1);

    cpu.a >>= 1;
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative(false);

    cycles
}

/// And memory with accumulator, rotate one bit to right and check bit 5 and 6:
/// If 5 and 6: set C, clear V
/// if !5 and !6: clear C, clear V
/// if 5 and !6: clear C, set V
/// if !5 and 6: set C, set V
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
/// * Overflow
/// * Zero
/// * Carry
pub fn arr(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Immediate => 2,
        _ => panic!("ARR doesn't support {:?} addressing", addressing),
    };

    cpu.a &= cpu.read_byte(&addressing, true);

    cpu.a = if cpu.flags.carry {
        cpu.a >> 1 | 1 << 7
    } else {
        cpu.a >> 1
    };

    let bit5 = (cpu.a & (1 << 5)) > 0;
    let bit6 = (cpu.a & (1 << 6)) > 0;

    cpu.flags.set_carry(bit6);
    cpu.flags.set_overflow(bit5 ^ bit6);
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);

    cycles
}

/// And memory with accumulator, then copy it to the X register
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
pub fn atx(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Immediate => 2,
        _ => panic!("ATX doesn't support {:?} addressing", addressing),
    };

    cpu.a &= cpu.read_byte(&addressing, true);
    cpu.x = cpu.a;

    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);

    cycles
}

/// And X register with accumulator, store the result in the X register and subtract the byte from
/// memory from the X register (without borrow)
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
pub fn axs(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Immediate => 2,
        _ => panic!("AXS doesn't support {:?} addressing", addressing),
    };

    cpu.x &= cpu.a;

    let byte = cpu.read_byte(&addressing, true);
    let (result, carry) = cpu.x.overflowing_sub(byte);
    cpu.x = result;

    cpu.flags.set_negative_from_byte(cpu.x);
    cpu.flags.set_zero_from_byte(cpu.x);
    cpu.flags.set_carry(carry);

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

    #[test]
    fn arr_ands_memory_then_rotates_right_and_sets_c_and_clears_v_if_bit_5_and_6_are_set() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0b1111_1111,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0b1100_0000])
            .expect("Failed to load ram");

        arr(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b0110_0000);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.overflow, false);
    }

    #[test]
    fn arr_ands_memory_then_rotates_right_and_clears_c_and_clears_v_if_bit_5_and_6_are_not_set() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0b1111_1111,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0b0000_0000])
            .expect("Failed to load ram");

        arr(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b0000_0000);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.overflow, false);
    }

    #[test]
    fn arr_ands_memory_then_rotates_right_and_clears_c_and_sets_v_if_only_bit_5_is_set() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0b1111_1111,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0b0100_0000])
            .expect("Failed to load ram");

        arr(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b0010_0000);
        assert_eq!(cpu.flags.carry, false);
        assert_eq!(cpu.flags.overflow, true);
    }

    #[test]
    fn arr_ands_memory_then_rotates_right_and_sets_c_and_sets_v_if_only_bit_6_is_set() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0b1111_1111,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0b1000_0000])
            .expect("Failed to load ram");

        arr(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b0100_0000);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.overflow, true);
    }

    #[test]
    fn atx_ands_memory_to_accumulator_then_copies_it_to_x() {
        let mut cpu = CPU {
            a: 0b1111_0000,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0b1010_1010])
            .expect("Failed to load ram");

        atx(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b1010_0000);
        assert_eq!(cpu.x, 0b1010_0000);
    }

    #[test]
    fn axs_ands_accumulator_with_x_and_subtracts_memory_from_x() {
        let mut cpu = CPU {
            a: 0b0000_1111,
            x: 0b0101_0101,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0b0000_0001])
            .expect("Failed to load ram");

        axs(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0b0000_1111);
        assert_eq!(cpu.x, 0b0000_0100);
    }
}
