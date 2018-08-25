use cpu::CPU;

/// Branch if Carry clear
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bcc(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.carry {
        2
    } else {
        cpu.offset_pc(u16::from(offset));
        3 // 4 if new page
    }
}

/// Branch if Carry set
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bcs(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.carry {
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
    fn bcc_applies_offset_if_carry_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.carry = false;

        bcc(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn bcc_doesnt_apply_offset_if_carry_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.carry = true;

        bcc(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn bcs_doesnt_apply_offset_if_carry_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.carry = false;

        bcs(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn bcs_applies_offset_if_carry_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0x05]);
        cpu.flags.carry = true;

        bcs(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }
}
