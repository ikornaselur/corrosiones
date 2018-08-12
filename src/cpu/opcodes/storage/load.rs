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
/// * Zero
/// * Negative
pub fn lda(cpu: &mut CPU, addressing: Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => {
            let address = cpu.read_next_double();
            cpu.a = cpu.read_byte(address);
            4
        }
        Addressing::AbsoluteX => {
            let address = cpu.read_next_double() + u16::from(cpu.x);
            cpu.a = cpu.read_byte(address);
            4 // Add 1 if page boundary is crossed
        }
        Addressing::AbsoluteY => {
            let address = cpu.read_next_double() + u16::from(cpu.y);
            cpu.a = cpu.read_byte(address);
            4 // Add 1 if page boundary is crossed
        }
        Addressing::Immediate => {
            cpu.a = cpu.read_next_byte();
            2
        }
        Addressing::IndirectX => {
            let ptr = u16::from(cpu.read_next_byte() + cpu.x);

            let address = cpu.read_double(ptr);

            cpu.a = cpu.read_byte(address);
            6
        }
        Addressing::IndirectY => {
            let ptr = u16::from(cpu.read_next_byte());

            let address = cpu.read_double(ptr) + u16::from(cpu.y);

            cpu.a = cpu.read_byte(address);
            5 // Add 1 if page boundary is crossed
        }
        Addressing::ZeroPage => {
            let address = cpu.read_next_byte() as u16;
            cpu.a = cpu.read_byte(address);
            3
        }
        Addressing::ZeroPageX => {
            let address = cpu.read_next_byte().wrapping_add(cpu.x) as u16;
            cpu.a = cpu.read_byte(address);
            4
        }
        _ => panic!("LDA doesn't support {:?} addressing", addressing),
    };

    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
    cycles
}

/// Load X Index with memory
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
/// * Zero
/// * Negative
pub fn ldx(cpu: &mut CPU, addressing: Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => {
            let address = cpu.read_next_double();
            cpu.x = cpu.read_byte(address);
            4
        }
        Addressing::AbsoluteX => {
            let address = cpu.read_next_double() + u16::from(cpu.x);
            cpu.x = cpu.read_byte(address);
            4 // Add 1 if page boundary is crossed
        }
        Addressing::Immediate => {
            cpu.x = cpu.read_next_byte();
            2
        }
        Addressing::ZeroPage => {
            let address = cpu.read_next_byte() as u16;
            cpu.x = cpu.read_byte(address);
            3
        }
        Addressing::ZeroPageX => {
            let address = cpu.read_next_byte().wrapping_add(cpu.x) as u16;
            cpu.x = cpu.read_byte(address);
            4
        }
        _ => panic!("LDX doesn't support {:?} addressing", addressing),
    };
    cpu.flags.set_zero_from_byte(cpu.x);
    cpu.flags.set_negative_from_byte(cpu.x);
    cycles
}

/// Load Y Index with memory
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
/// * Zero
/// * Negative
pub fn ldy(cpu: &mut CPU, addressing: Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => {
            let address = cpu.read_next_double();
            cpu.y = cpu.read_byte(address);
            4
        }
        Addressing::AbsoluteX => {
            let address = cpu.read_next_double() + u16::from(cpu.x);
            cpu.y = cpu.read_byte(address);
            4 // Add 1 if page boundary is crossed
        }
        Addressing::Immediate => {
            cpu.y = cpu.read_next_byte();
            2
        }
        Addressing::ZeroPage => {
            let address = cpu.read_next_byte() as u16;
            cpu.y = cpu.read_byte(address);
            3
        }
        Addressing::ZeroPageX => {
            let address = cpu.read_next_byte().wrapping_add(cpu.x) as u16;
            cpu.y = cpu.read_byte(address);
            4
        }
        _ => panic!("LDY doesn't support {:?} addressing", addressing),
    };
    cpu.flags.set_zero_from_byte(cpu.y);
    cpu.flags.set_negative_from_byte(cpu.y);
    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lda_immediate() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

        let cycles = lda(&mut cpu, Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0x03);
    }

    #[test]
    fn lda_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0xDE, 0x01, 0x00]);

        let cycles = lda(&mut cpu, Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.a, 0xDE);
    }

    #[test]
    fn lda_zeropage_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0xDE, 0x01, 0xAD]);

        let cycles = lda(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xAD);
    }

    #[test]
    fn lda_zeropage_x_wrapping() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0xFF,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xAA, 0xDE, 0x01, 0xAD]);

        let cycles = lda(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xAA);
    }

    #[test]
    fn lda_absolute() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA]);

        let cycles = lda(&mut cpu, Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xAA);
    }

    #[test]
    fn lda_absolute_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB]);

        let cycles = lda(&mut cpu, Addressing::AbsoluteX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xBB);
    }

    #[test]
    fn lda_absolute_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            y: 0x02,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB, 0xCC]);

        let cycles = lda(&mut cpu, Addressing::AbsoluteY);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xCC);
    }

    #[test]
    fn lda_indirect_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xAA, 0x04, 0xFF, 0xFF, 0x01, 0x00]);

        let cycles = lda(&mut cpu, Addressing::IndirectX);

        assert_eq!(cycles, 6);
        assert_eq!(cpu.a, 0xAA);
    }

    #[test]
    fn lda_indirect_y() {
        let mut cpu = CPU {
            pc: 0x0003,
            y: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xAA, 0x06, 0xFF, 0xFF, 0x01, 0x00]);

        let cycles = lda(&mut cpu, Addressing::IndirectY);

        assert_eq!(cycles, 5);
        assert_eq!(cpu.a, 0xAA);
    }

    #[test]
    fn ldx_immediate() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

        let cycles = ldx(&mut cpu, Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.x, 0x03);
    }

    #[test]
    fn ldx_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0xDE, 0x01, 0x00]);

        let cycles = ldx(&mut cpu, Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.x, 0xDE);
    }

    #[test]
    fn ldx_zeropage_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0xDE, 0x01, 0xAD]);

        let cycles = ldx(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.x, 0xAD);
    }

    #[test]
    fn ldx_zeropage_x_wrapping() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0xFF,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xAA, 0xDE, 0x01, 0xAD]);

        let cycles = ldx(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.x, 0xAA);
    }

    #[test]
    fn ldy_immediate() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);

        let cycles = ldy(&mut cpu, Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.y, 0x03);
    }

    #[test]
    fn ldy_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0xDE, 0x01, 0x00]);

        let cycles = ldy(&mut cpu, Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.y, 0xDE);
    }

    #[test]
    fn ldy_zeropage_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0xDE, 0x01, 0xAD]);

        let cycles = ldy(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.y, 0xAD);
    }

    #[test]
    fn ldy_zeropage_x_wrapping() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0xFF,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xAA, 0xDE, 0x01, 0xAD]);

        let cycles = ldy(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.y, 0xAA);
    }
}
