use cpu::{Addressing, CPU};

/// Jump to an address
///
/// *Note* there's a hardware bug in indirect addressing mode, when reading the second half of the
/// address, if it's in the next page, the second half will actually be read from the current
/// memory page instead of the next one (if first half is in 0x01FF, then second half will be read
/// from 0x0100 instead of 0x0200)
///
/// TODO Verify the bug is as I remember it and introduce it here
///
/// # Supported addressing modes
///
/// * Absolute - 3 cycles
/// * Indirect - 5 cycles
pub fn jmp(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 3,
        Addressing::Indirect => 5,
        _ => panic!("JMP doesn't support {:?} addressing", addressing),
    };

    let address = match addressing {
        Addressing::Absolute => cpu.read_next_double(true),
        Addressing::Indirect => {
            let indirect_addr = cpu.read_next_double(true);
            if indirect_addr as u8 == 0xFF {
                let lsb = cpu.raw_read_byte(indirect_addr);
                let msb = cpu.raw_read_byte(indirect_addr & !0xFF);
                (u16::from(msb) << 8) | u16::from(lsb)
            } else {
                cpu.read_double(indirect_addr)
            }
        }
        _ => panic!("JMP doesn't support {:?} addressing", addressing),
    };

    cpu.set_pc(address);

    cycles
}

/// Jump to subroutine
///
/// # Supported addressing modes
///
/// * Absolute - 6 cycles
pub fn jsr(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = 6;

    let address = match addressing {
        Addressing::Absolute => cpu.read_next_double(true),
        _ => panic!("JSR doesn't support {:?} addressing", addressing),
    };

    let return_addr = cpu.pc - 1;

    cpu.push_stack((return_addr >> 8) as u8);
    cpu.push_stack(return_addr as u8);

    cpu.set_pc(address);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn jmp_absolute() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xAD, 0xDE])
            .expect("Failed to load ram");

        jmp(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jmp_indirect() {
        let mut cpu = CPU {
            pc: 0x0003,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xAD, 0xDE, 0x01, 0x00])
            .expect("Failed to load ram");

        jmp(&mut cpu, &Addressing::Indirect);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jmp_indirect_boundary_bug() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xAA; 257])
            .expect("Failed to load ram");
        // The address to read from
        cpu.raw_write_byte(0x0002, 0xFF);
        cpu.raw_write_byte(0x0003, 0x00);
        // The PC bytes
        cpu.raw_write_byte(0x0000, 0x11);
        cpu.raw_write_byte(0x00FF, 0x22);
        cpu.raw_write_byte(0x0100, 0x33);

        jmp(&mut cpu, &Addressing::Indirect);

        assert_eq!(cpu.pc, 0x1122);
    }

    #[test]
    fn jst_stores_pc_on_stack() {
        let mut cpu = CPU {
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xAD, 0xDE, 0xFF])
            .expect("Failed to load ram");

        jsr(&mut cpu, &Addressing::Absolute);

        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.sp, 0xFB);
        assert_eq!(cpu.raw_read_byte(0x01FD), 0x00);
        assert_eq!(cpu.raw_read_byte(0x01FC), 0x02);
    }
}
