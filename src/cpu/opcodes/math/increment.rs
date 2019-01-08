use cpu::{Addressing, CPU};

/// Increment memory
///
/// # Supported addressing modes
///
/// * Absolute - 6 Cycles
/// * Absolute X - 7 Cycles
/// * Zero Page - 5 Cycles
/// * Zero Page X - 6 Cycles
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn inc(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX => 7,
        Addressing::ZeroPage => 5,
        _ => panic!("INC doesn't support {:?} addressing", addressing),
    };

    let (byte, _) = cpu.update_byte(&addressing, |x| (x.wrapping_add(1), None), true);

    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);

    cycles
}

/// Increment X register
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn inx(cpu: &mut CPU) -> u8 {
    let x = cpu.x.wrapping_add(1);
    cpu.x = x;

    cpu.flags.set_zero_from_byte(x);
    cpu.flags.set_negative_from_byte(x);

    2
}

/// Increment Y register
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn iny(cpu: &mut CPU) -> u8 {
    let y = cpu.y.wrapping_add(1);
    cpu.y = y;

    cpu.flags.set_zero_from_byte(y);
    cpu.flags.set_negative_from_byte(y);

    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inc_increments_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xA1, 0x01, 0x00])
            .expect("Failed to load ram");

        inc(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0xA2);
    }

    #[test]
    fn inc_increments_memory_wrapping() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0x00])
            .expect("Failed to load ram");

        inc(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0x00);
    }

    #[test]
    fn inx_increments_x_register() {
        let mut cpu = CPU {
            x: 0xA1,
            ..CPU::default()
        };

        inx(&mut cpu);

        assert_eq!(cpu.x, 0xA2);
    }

    #[test]
    fn inx_increments_x_register_wrapping() {
        let mut cpu = CPU {
            x: 0xFF,
            ..CPU::default()
        };

        inx(&mut cpu);

        assert_eq!(cpu.x, 0x00);
    }

    #[test]
    fn iny_increments_x_register() {
        let mut cpu = CPU {
            y: 0xA1,
            ..CPU::default()
        };

        iny(&mut cpu);

        assert_eq!(cpu.y, 0xA2);
    }

    #[test]
    fn iny_increments_x_register_wrapping() {
        let mut cpu = CPU {
            y: 0xFF,
            ..CPU::default()
        };

        iny(&mut cpu);

        assert_eq!(cpu.y, 0x00);
    }
}
