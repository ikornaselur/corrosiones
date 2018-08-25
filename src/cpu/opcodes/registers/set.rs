use cpu::CPU;

/// Set the carry flag
pub fn sec(cpu: &mut CPU) -> u8 {
    cpu.flags.set_carry(true);

    2
}

/// Set Interrupt Disable
pub fn sei(cpu: &mut CPU) -> u8 {
    cpu.flags.set_interrupt_disable(true);

    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sec_clears_carry_flag() {
        let mut cpu = CPU::new();

        sec(&mut cpu);

        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn sei_clears_interrupt_disable_flag() {
        let mut cpu = CPU::new();

        sei(&mut cpu);

        assert_eq!(cpu.flags.interrupt_disable, true);
    }
}
