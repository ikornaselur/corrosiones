use cpu::CPU;

/// Branch if Negative clear
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bpl(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.negative {
        2
    } else {
        cpu.offset_pc(offset as u16);
        3 // 4 if new page
    }
}

/// Branch if Negative set
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bmi(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.negative {
        cpu.offset_pc(offset as u16);
        3 // 4 if new page
    } else {
        2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bpl_applies_offset_if_negative_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.negative = false;

        bpl(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn bpl_doesnt_apply_offset_if_negative_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.negative = true;

        bpl(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn bmi_doesnt_apply_offset_if_negative_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.negative = false;

        bmi(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn bmi_applies_offset_if_negative_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.negative = true;

        bmi(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }
}
