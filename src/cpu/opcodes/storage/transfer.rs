use cpu::CPU;

/// Transfer accumulator to X index
///
/// # Flags affected
/// * Negative
/// * Zero
pub fn tax(cpu: &mut CPU) -> u8 {
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
    cpu.x = cpu.a;
    2
}

/// Transfer X index to accumulator
///
/// # Flags affected
/// * Negative
/// * Zero
pub fn txa(cpu: &mut CPU) -> u8 {
    cpu.flags.set_zero_from_byte(cpu.x);
    cpu.flags.set_negative_from_byte(cpu.x);
    cpu.a = cpu.x;
    2
}

/// Transfer accumulator to Y index
///
/// # Flags affected
/// * Negative
/// * Zero
pub fn tay(cpu: &mut CPU) -> u8 {
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
    cpu.y = cpu.a;
    2
}

/// Transfer Y index to accumulator
///
/// # Flags affected
/// * Negative
/// * Zero
pub fn tya(cpu: &mut CPU) -> u8 {
    cpu.flags.set_zero_from_byte(cpu.y);
    cpu.flags.set_negative_from_byte(cpu.y);
    cpu.a = cpu.y;
    2
}

/// Transfer stack pointer to X index
///
/// # Flags affected
/// * Negative
/// * Zero
pub fn tsx(cpu: &mut CPU) -> u8 {
    cpu.flags.set_zero_from_byte(cpu.sp);
    cpu.flags.set_negative_from_byte(cpu.sp);
    cpu.x = cpu.sp;
    2
}

/// Transfer X index to stack pointer
///
/// # Flags affected
/// * Negative
/// * Zero
pub fn txs(cpu: &mut CPU) -> u8 {
    cpu.flags.set_zero_from_byte(cpu.x);
    cpu.flags.set_negative_from_byte(cpu.x);
    cpu.sp = cpu.x;
    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tax_transfers_a_to_x() {
        let mut cpu = CPU {
            a: 0xAB,
            ..CPU::default()
        };

        let cycles = tax(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.x, 0xAB);
    }

    #[test]
    fn tax_sets_zero_flag() {
        let mut cpu = CPU {
            a: 0,
            x: 0xFF,
            ..CPU::default()
        };

        tax(&mut cpu);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn tax_sets_negative_flag() {
        let mut cpu = CPU {
            a: 0b1000_0000,
            ..CPU::default()
        };

        tax(&mut cpu);

        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn txa_transfers_x_to_a() {
        let mut cpu = CPU {
            x: 0xAB,
            ..CPU::default()
        };

        let cycles = txa(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0xAB);
    }

    #[test]
    fn txa_sets_zero_flag() {
        let mut cpu = CPU {
            x: 0,
            a: 0xFF,
            ..CPU::default()
        };

        txa(&mut cpu);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn txa_sets_negative_flag() {
        let mut cpu = CPU {
            x: 0b1000_0000,
            ..CPU::default()
        };

        txa(&mut cpu);

        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn tay_transfers_a_to_y() {
        let mut cpu = CPU {
            a: 0xAB,
            ..CPU::default()
        };

        let cycles = tay(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.y, 0xAB);
    }

    #[test]
    fn tay_sets_zero_flag() {
        let mut cpu = CPU {
            a: 0,
            y: 0xFF,
            ..CPU::default()
        };

        tay(&mut cpu);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn tay_sets_negative_flag() {
        let mut cpu = CPU {
            a: 0b1000_0000,
            ..CPU::default()
        };

        tay(&mut cpu);

        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn tya_transfers_y_to_a() {
        let mut cpu = CPU {
            y: 0xAB,
            ..CPU::default()
        };

        let cycles = tya(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0xAB);
    }

    #[test]
    fn tya_sets_zero_flag() {
        let mut cpu = CPU {
            y: 0,
            a: 0xFF,
            ..CPU::default()
        };

        tya(&mut cpu);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn tya_sets_negative_flag() {
        let mut cpu = CPU {
            y: 0b1000_0000,
            ..CPU::default()
        };

        tya(&mut cpu);

        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn tsx_transfers_sp_to_x() {
        let mut cpu = CPU {
            sp: 0xAB,
            ..CPU::default()
        };

        let cycles = tsx(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.x, 0xAB);
    }

    #[test]
    fn tsx_sets_zero_flag() {
        let mut cpu = CPU {
            sp: 0,
            x: 0xFF,
            ..CPU::default()
        };

        tsx(&mut cpu);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn tsx_sets_negative_flag() {
        let mut cpu = CPU {
            sp: 0b1000_0000,
            ..CPU::default()
        };

        tsx(&mut cpu);

        assert_eq!(cpu.flags.negative, true);
    }

    #[test]
    fn txs_transfers_x_to_sp() {
        let mut cpu = CPU {
            x: 0xAB,
            ..CPU::default()
        };

        let cycles = txs(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.sp, 0xAB);
    }

    #[test]
    fn txs_sets_zero_flag() {
        let mut cpu = CPU {
            x: 0,
            sp: 0xFF,
            ..CPU::default()
        };

        txs(&mut cpu);

        assert_eq!(cpu.flags.zero, true);
    }

    #[test]
    fn txs_sets_negative_flag() {
        let mut cpu = CPU {
            x: 0b1000_0000,
            ..CPU::default()
        };

        txs(&mut cpu);

        assert_eq!(cpu.flags.negative, true);
    }
}
