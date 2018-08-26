use cpu::CPU;

/// Push Accumulator onto the stack
pub fn pha(cpu: &mut CPU) -> u8 {
    let cycles = 3;

    let acc = cpu.a;
    cpu.push_stack(acc);

    cycles
}

/// Push flags onto the stack
pub fn php(cpu: &mut CPU) -> u8 {
    let cycles = 3;

    let flags = cpu.flags.as_byte();
    cpu.push_stack(flags);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pha_pushes_accumulator_on_stack() {
        let mut cpu = CPU {
            sp: 0xFF,
            a: 0xAB,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");

        pha(&mut cpu);

        assert_eq!(cpu.raw_read_byte(0x01FF), 0xAB);
    }

    #[test]
    fn php_pushes_flags_on_stack() {
        let mut cpu = CPU {
            sp: 0xFF,
            a: 0xAB,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.flags.set_carry(true);
        cpu.flags.set_zero(true);
        cpu.flags.set_overflow(true);
        cpu.flags.set_negative(true);

        php(&mut cpu);

        assert_eq!(cpu.raw_read_byte(0x01FF), cpu.flags.as_byte());
    }
}
