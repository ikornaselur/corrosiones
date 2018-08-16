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
    fn tay_transfers_a_to_x() {
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
}
