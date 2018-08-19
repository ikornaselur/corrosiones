use cpu::{Addressing, CPU};

/// Jump to an address
///
/// # Supported addressing modes
///
/// * Absolute - 3 cycles
/// * Indirect - 5 cycles
pub fn jmp(cpu: &mut CPU, addressing: Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute => 3,
        Addressing::Indirect => 5,
        _ => panic!("JMP doesn't support {:?} addressing", addressing),
    };

    let address = match addressing {
        Addressing::Absolute => cpu.read_next_double(true),
        Addressing::Indirect => {
            let indirect_addr = cpu.read_next_double(true);
            cpu.read_double(indirect_addr)
        }
        _ => panic!("JMP doesn't support {:?} addressing", addressing),
    };

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
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0xAD, 0xDE]);

        jmp(&mut cpu, Addressing::Absolute);

        assert_eq!(cpu.pc, 0xDEAD);
    }

    #[test]
    fn jmp_indirect() {
        let mut cpu = CPU {
            pc: 0x0003,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xAD, 0xDE, 0x01, 0x00]);

        jmp(&mut cpu, Addressing::Indirect);

        assert_eq!(cpu.pc, 0xDEAD);
    }
}
