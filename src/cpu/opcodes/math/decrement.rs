use cpu::{Addressing, CPU};

/// Decrement memory
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
pub fn dec(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 6,
        Addressing::AbsoluteX => 7,
        Addressing::ZeroPage => 5,
        _ => panic!("DEC doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.update_byte(&addressing, |x| x.wrapping_sub(1), true);

    cpu.flags.set_zero_from_byte(byte);
    cpu.flags.set_negative_from_byte(byte);

    cycles
}

/// Decrement X Index
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn dex(cpu: &mut CPU) -> u8 {
    let x = cpu.x.wrapping_sub(1);
    cpu.x = x;

    cpu.flags.set_zero_from_byte(x);
    cpu.flags.set_negative_from_byte(x);

    2
}

/// Decrement Y Index
///
/// # Flags affected
///
/// * Negative
/// * Zero
pub fn dey(cpu: &mut CPU) -> u8 {
    let y = cpu.y.wrapping_sub(1);
    cpu.y = y;

    cpu.flags.set_zero_from_byte(y);
    cpu.flags.set_negative_from_byte(y);

    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dec_decrements_memory() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xA1, 0x01, 0x00]);

        dec(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0xA0);
    }

    #[test]
    fn dec_decrements_memory_wrapping() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0x00, 0x01, 0x00]);

        dec(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.raw_read_byte(0x0001), 0xFF);
    }

    #[test]
    fn dex_decrements_x_index() {
        let mut cpu = CPU {
            x: 0xA1,
            ..CPU::default()
        };

        dex(&mut cpu);

        assert_eq!(cpu.x, 0xA0);
    }

    #[test]
    fn dex_decrements_x_index_wrapping() {
        let mut cpu = CPU {
            x: 0x00,
            ..CPU::default()
        };

        dex(&mut cpu);

        assert_eq!(cpu.x, 0xFF);
    }

    #[test]
    fn dey_decrements_x_index() {
        let mut cpu = CPU {
            y: 0xA1,
            ..CPU::default()
        };

        dey(&mut cpu);

        assert_eq!(cpu.y, 0xA0);
    }

    #[test]
    fn dey_decrements_x_index_wrapping() {
        let mut cpu = CPU {
            y: 0x00,
            ..CPU::default()
        };

        dey(&mut cpu);

        assert_eq!(cpu.y, 0xFF);
    }
}
