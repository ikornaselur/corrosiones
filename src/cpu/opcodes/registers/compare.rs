use cpu::{Addressing, CPU};

/// Compare accumulator to memory
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
/// * Zero
pub fn cmp(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute
        | Addressing::AbsoluteX
        | Addressing::AbsoluteY
        | Addressing::ZeroPageX => 4,
        Addressing::Immediate => 2,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        _ => panic!("CMP doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    cpu.flags.set_carry(byte <= cpu.a);
    cpu.flags.set_zero(byte == cpu.a);
    cpu.flags.set_negative((cpu.a.wrapping_sub(byte) >> 7) > 0);

    cycles
}

/// Compare X register to memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Immediate - 2 cycles
/// * Zero Page - 3 Cycles
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Zero
pub fn cpx(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 4,
        Addressing::Immediate => 2,
        Addressing::ZeroPage => 3,
        _ => panic!("CPX doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    cpu.flags.set_carry(byte <= cpu.x);
    cpu.flags.set_zero(byte == cpu.x);
    cpu.flags.set_negative((cpu.x.wrapping_sub(byte) >> 7) > 0);

    cycles
}

/// Compare Y register to memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Immediate - 2 cycles
/// * Zero Page - 3 Cycles
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Zero
pub fn cpy(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 4,
        Addressing::Immediate => 2,
        Addressing::ZeroPage => 3,
        _ => panic!("CPY doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    cpu.flags.set_carry(byte <= cpu.y);
    cpu.flags.set_zero(byte == cpu.y);
    cpu.flags.set_negative((cpu.y.wrapping_sub(byte) >> 7) > 0);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp_sets_carry_if_accumulator_higher_than_memory() {
        let mut cpu = CPU {
            a: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x50, 0x10])
            .expect("Failed to load ram");

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, false); // A < M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, true); // A == M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, false); // A < M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, true); // A > M
    }

    #[test]
    fn cmp_sets_zero_if_accumulator_equals_memory() {
        let mut cpu = CPU {
            a: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x10])
            .expect("Failed to load ram");

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A < M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, true); // A == M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A > M
    }

    #[test]
    fn cmp_sets_negative_if_byte_is_higher_than_accumulator() {
        let mut cpu = CPU {
            a: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };

        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x10])
            .expect("Failed to load ram");

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, true); // A < M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false); // A == M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false); // A > M
    }

    #[test]
    fn cpx_sets_carry_if_a_higher_than_memory() {
        let mut cpu = CPU {
            x: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x50, 0x10])
            .expect("Failed to load ram");

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, false); // A < M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, true); // A == M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, false); // A < M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, true); // A > M
    }

    #[test]
    fn cpx_sets_zero_if_x_equals_memory() {
        let mut cpu = CPU {
            x: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x10])
            .expect("Failed to load ram");

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A < M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, true); // A == M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A > M
    }

    #[test]
    fn cpx_sets_negative_if_memory_is_higher_than_x() {
        let mut cpu = CPU {
            x: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x10])
            .expect("Failed to load ram");

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, true); // A < M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false); // A == M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false); // A > M
    }

    #[test]
    fn cpy_sets_carry_if_a_higher_than_memory() {
        let mut cpu = CPU {
            y: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x50, 0x10])
            .expect("Failed to load ram");

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, false); // A < M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, true); // A == M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, false); // A < M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.carry, true); // A > M
    }

    #[test]
    fn cpy_sets_zero_if_x_equals_memory() {
        let mut cpu = CPU {
            y: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x10])
            .expect("Failed to load ram");

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A < M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, true); // A == M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A > M
    }

    #[test]
    fn cpy_sets_negative_if_memory_negative() {
        let mut cpu = CPU {
            y: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0x50, 0x30, 0x10])
            .expect("Failed to load ram");

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, true); // A < M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false); // A == M

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false); // A > M
    }
}
