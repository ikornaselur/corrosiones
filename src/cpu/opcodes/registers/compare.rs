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
        _ => panic!("cmp doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    let acc = cpu.a;
    cpu.flags.set_carry(byte <= acc);
    cpu.flags.set_zero(byte == acc);
    cpu.flags.set_negative_from_byte(byte);

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
        _ => panic!("cpx doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    let x = cpu.x;
    cpu.flags.set_carry(byte <= x);
    cpu.flags.set_zero(byte == x);
    cpu.flags.set_negative_from_byte(byte);

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
        _ => panic!("cpy doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);

    let y = cpu.y;
    cpu.flags.set_carry(byte <= y);
    cpu.flags.set_zero(byte == y);
    cpu.flags.set_negative_from_byte(byte);

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
        cpu.memory.load_ram(vec![0xFF, 0x50, 0x30, 0x50, 0x10]);

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
        cpu.memory.load_ram(vec![0xFF, 0x50, 0x30, 0x10]);

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A < M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, true); // A == M

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A > M
    }

    #[test]
    fn cmp_sets_negative_if_memory_negative() {
        let mut cpu = CPU {
            a: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0b1000_0000, 0b0111_1111]);

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, true);

        cmp(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cpx_sets_carry_if_a_higher_than_memory() {
        let mut cpu = CPU {
            x: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0x50, 0x30, 0x50, 0x10]);

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
        cpu.memory.load_ram(vec![0xFF, 0x50, 0x30, 0x10]);

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A < M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, true); // A == M

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.zero, false); // A > M
    }

    #[test]
    fn cpx_sets_negative_if_memory_negative() {
        let mut cpu = CPU {
            x: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0b1000_0000, 0b0111_1111]);

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, true);

        cpx(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false);
    }

    #[test]
    fn cpy_sets_carry_if_a_higher_than_memory() {
        let mut cpu = CPU {
            y: 0x30,
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0x50, 0x30, 0x50, 0x10]);

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
        cpu.memory.load_ram(vec![0xFF, 0x50, 0x30, 0x10]);

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
        cpu.memory.load_ram(vec![0xFF, 0b1000_0000, 0b0111_1111]);

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, true);

        cpy(&mut cpu, &Addressing::Immediate);
        assert_eq!(cpu.flags.negative, false);
    }
}
