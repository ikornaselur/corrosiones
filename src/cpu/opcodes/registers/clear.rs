use cpu::CPU;

/// Clear the carry flag
pub fn clc(cpu: &mut CPU) -> u8 {
    cpu.flags.set_carry(false);

    2
}

/// Clear Interrupt Disable
pub fn cli(cpu: &mut CPU) -> u8 {
    cpu.flags.set_interrupt_disable(false);

    2
}

/// Clear Overflow
pub fn clv(cpu: &mut CPU) -> u8 {
    cpu.flags.set_overflow(false);

    2
}

/// Clear Decimal
pub fn cld(cpu: &mut CPU) -> u8 {
    cpu.flags.set_decimal(false);

    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clc_clears_carry_flag() {
        let mut cpu = CPU::new();
        cpu.flags.set_carry(true);

        clc(&mut cpu);

        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn cli_clears_interrupt_disable_flag() {
        let mut cpu = CPU::new();
        cpu.flags.set_interrupt_disable(true);

        cli(&mut cpu);

        assert_eq!(cpu.flags.interrupt_disable, false);
    }

    #[test]
    fn clv_clears_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.flags.set_overflow(true);

        clv(&mut cpu);

        assert_eq!(cpu.flags.overflow, false);
    }

    #[test]
    fn cld_clears_decimal_flag() {
        let mut cpu = CPU::new();
        cpu.flags.set_decimal(true);

        cld(&mut cpu);

        assert_eq!(cpu.flags.decimal, false);
    }
}
