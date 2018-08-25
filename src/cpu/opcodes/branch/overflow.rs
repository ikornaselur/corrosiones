use cpu::CPU;

/// Branch if Overflow clear
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bvc(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.overflow {
        2
    } else {
        cpu.offset_pc(u16::from(offset));
        3 // 4 if new page
    }
}

/// Branch if Overflow set
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bvs(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.overflow {
        cpu.offset_pc(u16::from(offset));
        3 // 4 if new page
    } else {
        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bvc_applies_offset_if_overflow_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.overflow = false;

        bvc(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn bvc_doesnt_apply_offset_if_overflow_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.overflow = true;

        bvc(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn bvs_doesnt_apply_offset_if_overflow_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.overflow = false;

        bvs(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn bvs_applies_offset_if_overflow_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.overflow = true;

        bvs(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }
}
