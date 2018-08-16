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
/// None
pub fn sta(cpu: &mut CPU, addressing: Addressing) -> u8 {
    match addressing {
        Addressing::Absolute => {
            let address = cpu.read_next_double();
            let acc = cpu.a;
            cpu.write_byte(address, acc);
            4
        }
        Addressing::AbsoluteX => {
            let address = cpu.read_next_double() + u16::from(cpu.x);
            let acc = cpu.a;
            cpu.write_byte(address, acc);
            4 // Add 1 if page boundary is crossed
        }
        Addressing::AbsoluteY => {
            let address = cpu.read_next_double() + u16::from(cpu.y);
            let acc = cpu.a;
            cpu.write_byte(address, acc);
            4 // Add 1 if page boundary is crossed
        }
        Addressing::IndirectX => {
            let ptr = u16::from(cpu.read_next_byte() + cpu.x);

            let address = cpu.read_double(ptr);

            let acc = cpu.a;
            cpu.write_byte(address, acc);
            6
        }
        Addressing::IndirectY => {
            let ptr = u16::from(cpu.read_next_byte());

            let address = cpu.read_double(ptr) + u16::from(cpu.y);

            let acc = cpu.a;
            cpu.write_byte(address, acc);
            5 // Add 1 if page boundary is crossed
        }
        Addressing::ZeroPage => {
            let address = cpu.read_next_byte() as u16;

            let acc = cpu.a;
            cpu.write_byte(address, acc);
            3
        }
        Addressing::ZeroPageX => {
            let address = cpu.read_next_byte().wrapping_add(cpu.x) as u16;

            let acc = cpu.a;
            cpu.write_byte(address, acc);
            4
        }
        _ => panic!("STA doesn't support {:?} addressing", addressing),
    }
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
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0xFF, 0x01, 0x00]);

        let cycles = sta(&mut cpu, Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.read_byte(0x0001), 0xAB);
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
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB]);

        let cycles = sta(&mut cpu, Addressing::AbsoluteX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.read_byte(0x0006), 0xAB);
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
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB]);

        let cycles = sta(&mut cpu, Addressing::AbsoluteY);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.read_byte(0x0006), 0xAB);
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
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0x01]);

        let cycles = sta(&mut cpu, Addressing::IndirectX);

        assert_eq!(cycles, 6);
        assert_eq!(cpu.read_byte(0x0001), 0xAB);
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
            .load_ram(vec![0xFF, 0xFF, 0xFF, 0x07, 0x00, 0xFF, 0xAA, 0x01]);

        let cycles = sta(&mut cpu, Addressing::IndirectY);

        assert_eq!(cycles, 5);
        assert_eq!(cpu.read_byte(0x0002), 0xAB);
    }

    #[test]
    fn sta_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x01, 0xFF]);

        let cycles = sta(&mut cpu, Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.read_byte(0x0001), 0xAB);
    }

    #[test]
    fn sta_zeropage_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0xAB,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x01, 0xFF]);

        let cycles = sta(&mut cpu, Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.read_byte(0x0003), 0xAB);
    }
}
