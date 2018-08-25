use cpu::CPU;

/// Pull Accumulator from the stack
pub fn pla(cpu: &mut CPU) -> u8 {
    let cycles = 3;

    let acc = cpu.pop_stack();
    cpu.a = acc;

    cycles
}

/// Pull flags from the stack
pub fn plp(cpu: &mut CPU) -> u8 {
    let cycles = 3;

    let flags = cpu.pop_stack();
    cpu.flags.set_from_byte(flags);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pla_pulls_accumulator_from_stack() {
        let mut cpu = CPU {
            sp: 0xFE,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x01FF, 0xAB);

        pla(&mut cpu);

        assert_eq!(cpu.a, 0xAB);
    }

    #[test]
    fn plp_pulls_flags_from_stack() {
        let mut cpu = CPU {
            sp: 0xFE,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x01FF, 0b1100_0011);

        plp(&mut cpu);

        assert_eq!(cpu.flags.carry, true);
        assert_eq!(cpu.flags.zero, true);
        assert_eq!(cpu.flags.overflow, true);
        assert_eq!(cpu.flags.negative, true);
    }
}
