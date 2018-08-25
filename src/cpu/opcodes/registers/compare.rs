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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp_sets_carry_if_a_higher_than_memory() {
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
}
