use cpu::{Addressing, CPU};

/// Load accumulator with memory
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
pub fn lda(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute
        | Addressing::AbsoluteX
        | Addressing::AbsoluteY
        | Addressing::ZeroPageX => 4,
        Addressing::Immediate => 2,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        _ => panic!("LDA doesn't support {:?} addressing", addressing),
    };

    cpu.a = cpu.read_byte(&addressing, true);
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
    cycles
}

/// Load X register with memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Absolute Y - 4* Cycles
/// * Immediate - 2 Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page Y - 4 Cycles
///
/// \* Add 1 if page boundary is crossed
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn ldx(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::AbsoluteY | Addressing::ZeroPageY => 4,
        Addressing::Immediate => 2,
        Addressing::ZeroPage => 3,
        _ => panic!("LDX doesn't support {:?} addressing", addressing),
    };
    cpu.x = cpu.read_byte(&addressing, true);
    cpu.flags.set_zero_from_byte(cpu.x);
    cpu.flags.set_negative_from_byte(cpu.x);
    cycles
}

/// Load X register and Accumulator with memory
///
/// *Undocumented instruction*
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Absolute Y - 4* Cycles
/// * Immediate - 2 Cycles
/// * Indirect X - 6 Cycles
/// * Indirect Y - 5* Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page Y - 4 Cycles
pub fn lax(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::AbsoluteY | Addressing::ZeroPageY => 4,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        Addressing::Immediate => 2,
        _ => panic!("LAX doesn't support {:?} addressing", addressing),
    };
    let byte = cpu.read_byte(&addressing, true);
    cpu.x = byte;
    cpu.a = byte;
    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);
    cycles
}

/// Load Y register with memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Absolute X - 4* Cycles
/// * Immediate - 2 Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page X - 4 Cycles
///
/// \* Add 1 if page boundary is crossed
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn ldy(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::AbsoluteX | Addressing::ZeroPageX => 4,
        Addressing::Immediate => 2,
        Addressing::ZeroPage => 3,
        _ => panic!("LDY doesn't support {:?} addressing", addressing),
    };
    cpu.y = cpu.read_byte(&addressing, true);
    cpu.flags.set_zero_from_byte(cpu.y);
    cpu.flags.set_negative_from_byte(cpu.y);
    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lda_loads_accumulator() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06])
            .expect("Failed to load ram");

        let cycles = lda(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0x03);
    }

    #[test]
    fn ldx_loads_x_register() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06])
            .expect("Failed to load ram");

        let cycles = ldx(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.x, 0x03);
    }

    #[test]
    fn lax_loads_both_accumulator_and_x_register() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0xAB, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = lax(&mut cpu, &Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.x, 0xAB);
        assert_eq!(cpu.a, 0xAB);
    }

    #[test]
    fn ldy_loads_y_register() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06])
            .expect("Failed to load ram");

        let cycles = ldy(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.y, 0x03);
    }
}
