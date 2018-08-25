use cpu::CPU;

/// Branch if Zero clear
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn bne(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.zero {
        2
    } else {
        cpu.offset_pc(u16::from(offset));
        3 // 4 if new page
    }
}

/// Branch if Zero set
///
/// # Cycles
///
/// * 2 if branch not taken
/// * 3 if branch taken
/// * 4 if branch taken to a new page
pub fn beq(cpu: &mut CPU) -> u8 {
    let offset = cpu.read_next_byte(true);

    if cpu.flags.zero {
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
    fn bne_applies_offset_if_zero_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05])
            .expect("Failed to load ram");
        cpu.flags.zero = false;

        bne(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }

    #[test]
    fn bne_doesnt_apply_offset_if_zero_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05])
            .expect("Failed to load ram");
        cpu.flags.zero = true;

        bne(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn beq_doesnt_apply_offset_if_zero_not_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05])
            .expect("Failed to load ram");
        cpu.flags.zero = false;

        beq(&mut cpu);

        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn beq_applies_offset_if_zero_is_set() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05])
            .expect("Failed to load ram");
        cpu.flags.zero = true;

        beq(&mut cpu);

        assert_eq!(cpu.pc, 0x0008);
    }
}
