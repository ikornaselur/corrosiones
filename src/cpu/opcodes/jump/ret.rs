use cpu::CPU;

/// Return from interrupt
pub fn rti(cpu: &mut CPU) -> u8 {
    let cycles = 6;

    let flags = cpu.pop_stack();
    let lsb = cpu.pop_stack();
    let msb = cpu.pop_stack();

    cpu.set_pc((u16::from(msb) << 8) | u16::from(lsb));
    cpu.flags.set_from_byte(flags);

    cycles
}

/// Return from Subroutine
pub fn rts(cpu: &mut CPU) -> u8 {
    let cycles = 6;

    let lsb = cpu.pop_stack();
    let msb = cpu.pop_stack();

    let pc = ((u16::from(msb) << 8) | u16::from(lsb)).wrapping_add(1);

    cpu.set_pc(pc);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rti_restores_pc_and_flags() {
        let mut cpu = CPU {
            sp: 0xF0,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x01F1, 0b1100_0011);
        cpu.raw_write_byte(0x01F2, 0xAD);
        cpu.raw_write_byte(0x01F3, 0xDE);

        rti(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.sp, 0xF3);
        assert_eq!(cpu.flags.negative, true);
        assert_eq!(cpu.flags.overflow, true);
        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn rts_restores_pc() {
        let mut cpu = CPU {
            sp: 0xF0,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x01F1, 0xAC);
        cpu.raw_write_byte(0x01F2, 0xDE);

        rts(&mut cpu);

        assert_eq!(cpu.pc, 0xDEAD);
        assert_eq!(cpu.sp, 0xF2);
    }
}
