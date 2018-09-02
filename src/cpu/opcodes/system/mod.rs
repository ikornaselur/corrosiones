use cpu::{Addressing, CPU};

/// A No-op
///
/// The double argument makes the CPU skip the next byte, this is an *undocumented feature*
pub fn nop(cpu: &mut CPU, size: usize, addressing: &Addressing) -> u8 {
    cpu.offset_pc(size as u8);

    match addressing {
        Addressing::Immediate => 2,
        Addressing::ZeroPage => 3,
        Addressing::ZeroPageX => 4,
        Addressing::Absolute => 4,
        Addressing::AbsoluteX => 4, // Extra cycle if cross border
        _ => panic!("NOP doesn't support {:?} addressing", addressing),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nop_size() {
        let mut cpu = CPU::new();

        cpu.memory
            .load_ram(vec![0xFF; 3])
            .expect("Failed to load ram");

        nop(&mut cpu, 0, &Addressing::Immediate);
        assert_eq!(cpu.pc, 0);

        nop(&mut cpu, 1, &Addressing::Immediate);
        assert_eq!(cpu.pc, 1);

        nop(&mut cpu, 2, &Addressing::Immediate);
        assert_eq!(cpu.pc, 3);
    }
}
