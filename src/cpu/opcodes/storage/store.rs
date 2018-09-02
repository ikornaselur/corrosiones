use cpu::{Addressing, CPU};

/// Store accumulator in memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Absolute X - 4* Cycles
/// * Absolute Y - 4* Cycles
/// * Indirect X - 6 Cycles
/// * Indirect Y - 5* Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page X - 4 Cycles
///
/// \* Add 1 if page boundary is crossed
///
/// # Flags affected
///
/// None
pub fn sta(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute
        | Addressing::AbsoluteX
        | Addressing::AbsoluteY
        | Addressing::ZeroPageX => 4,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        _ => panic!("STA doesn't support {:?} addressing", addressing),
    };

    let acc = cpu.a;
    cpu.write_byte(&addressing, acc, true);
    cycles
}

/// Store the X index in memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page Y - 4 Cycles
///
/// # Flags affected
///
/// None
pub fn stx(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageY => 4,
        Addressing::ZeroPage => 3,
        _ => panic!("STX doesn't support {:?} addressing", addressing),
    };

    let x = cpu.x;
    cpu.write_byte(&addressing, x, true);
    cycles
}

/// Store the Y index in memory
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page X - 4 Cycles
///
/// # Flags affected
///
/// None
pub fn sty(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute | Addressing::ZeroPageX => 4,
        Addressing::ZeroPage => 3,
        _ => panic!("STY doesn't support {:?} addressing", addressing),
    };

    let y = cpu.y;
    cpu.write_byte(&addressing, y, true);
    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sta_absolute() {
        let mut cpu = CPU {
            pc: 0x0003,
            a: 0xAB,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xFF, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn sta_absolute_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            x: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::AbsoluteX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0006), 0xAB);
    }

    #[test]
    fn sta_absolute_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            y: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::AbsoluteY);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0006), 0xAB);
    }

    #[test]
    fn sta_indirect_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            x: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0x01])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::IndirectX);

        assert_eq!(cycles, 6);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn sta_indirect_y() {
        let mut cpu = CPU {
            pc: 0x0003,
            a: 0xAB,
            y: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xFF, 0x07, 0x00, 0xFF, 0xAA, 0x01])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::IndirectY);

        assert_eq!(cycles, 5);
        assert_eq!(cpu.raw_read_byte(0x0002), 0xAB);
    }

    #[test]
    fn sta_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0xFF])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn sta_zeropage_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0xFF])
            .expect("Failed to load ram");

        let cycles = sta(&mut cpu, &Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0003), 0xAB);
    }

    #[test]
    fn stx_absolute() {
        let mut cpu = CPU {
            pc: 0x0003,
            x: 0xAB,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xFF, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = stx(&mut cpu, &Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn stx_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0xAB,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0xFF])
            .expect("Failed to load ram");

        let cycles = stx(&mut cpu, &Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn stx_zeropage_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0xAB,
            y: 0x02,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0xFF])
            .expect("Failed to load ram");

        let cycles = stx(&mut cpu, &Addressing::ZeroPageY);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0003), 0xAB);
    }

    #[test]
    fn sty_absolute() {
        let mut cpu = CPU {
            pc: 0x0003,
            y: 0xAB,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xFF, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = sty(&mut cpu, &Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn sty_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            y: 0xAB,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0xFF])
            .expect("Failed to load ram");

        let cycles = sty(&mut cpu, &Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.raw_read_byte(0x0001), 0xAB);
    }

    #[test]
    fn sty_zeropage_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            y: 0xAB,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x01, 0xFF])
            .expect("Failed to load ram");

        let cycles = sty(&mut cpu, &Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.raw_read_byte(0x0003), 0xAB);
    }
}
