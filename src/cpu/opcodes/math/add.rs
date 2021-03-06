use cpu::utils::get_overflow;
use cpu::{Addressing, CPU};

pub(super) fn add_byte_to_accumulator(cpu: &mut CPU, original_byte: u8) {
    let (byte, byte_carry) = if cpu.flags.carry {
        original_byte.overflowing_add(1)
    } else {
        (original_byte, false)
    };

    let (result, carry) = cpu.a.overflowing_add(byte);

    let overflow = get_overflow(original_byte, cpu.a, result);
    cpu.flags.set_overflow(overflow);

    cpu.a = result;
    cpu.flags.set_carry(carry || byte_carry);
    cpu.flags.set_zero_from_byte(cpu.a);
    cpu.flags.set_negative_from_byte(cpu.a);
}

/// Add with carry
///
/// # Supported addressing modes
///
/// * Absolute - 4 Cycles
/// * Absolute X - 4* Cycles
/// * Absolute Y - 4* Cycles
/// * Immediate - 2 cycles
/// * Indirect X - 6 Cycles
/// * Indirect Y - 5* Cycles
/// * Zero Page - 3 Cycles
/// * Zero Page X - 4 Cycles
///
/// \* Add 1 if page boundary is crossed
///
/// # Flags affected
///
/// * Carry
/// * Negative
/// * Overflow
/// * Zero
pub fn adc(cpu: &mut CPU, addressing: &Addressing) -> u8 {
    let cycles = match addressing {
        Addressing::Absolute
        | Addressing::AbsoluteX
        | Addressing::AbsoluteY
        | Addressing::ZeroPageX => 4,
        Addressing::Immediate => 2,
        Addressing::IndirectX => 6,
        Addressing::IndirectY => 5,
        Addressing::ZeroPage => 3,
        _ => panic!("ADC doesn't support {:?} addressing", addressing),
    };

    let byte = cpu.read_byte(&addressing, true);
    add_byte_to_accumulator(cpu, byte);

    cycles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adc_immediate_without_carry() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 1,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0x01, 0x02, 0x03])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 2);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_immediate_with_carry() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 1,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0x01, 0x02, 0x03])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        let cycles = adc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 3);
        assert_eq!(cpu.flags.carry, false);
    }

    #[test]
    fn adc_immediate_with_carry_and_max_value() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 1,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0xFF, 0x02, 0x03])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        let cycles = adc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 1);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn adc_immediate_with_carry_and_max_value_and_max_accumulator() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0xFF,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0xFF, 0x02, 0x03])
            .expect("Failed to load ram");
        cpu.flags.set_carry(true);

        let cycles = adc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn adc_immediate_without_carry_and_max_value_and_max_accumulator() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 0xFF,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0xFF, 0x02, 0x03])
            .expect("Failed to load ram");
        cpu.flags.set_carry(false);

        let cycles = adc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.a, 0xFE);
        assert_eq!(cpu.flags.carry, true);
    }

    #[test]
    fn adc_twos_complement_minus_1_plus_1() {
        let mut cpu = CPU {
            pc: 0x0001,
            a: 1,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, -1i8 as u8])
            .expect("Failed to load ram");

        adc(&mut cpu, &Addressing::Immediate);

        assert_eq!(cpu.a, 0);
    }

    #[test]
    fn adc_absolute() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0x11,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::Absolute);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xBB);
    }

    #[test]
    fn adc_absolute_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0x11,
            x: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::AbsoluteX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xCC);
    }

    #[test]
    fn adc_absolute_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0x11,
            y: 0x02,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA, 0xBB, 0xCC])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::AbsoluteY);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xDD);
    }

    #[test]
    fn adc_indirect_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0x11,
            x: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xAA, 0x04, 0xFF, 0xFF, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::IndirectX);

        assert_eq!(cycles, 6);
        assert_eq!(cpu.a, 0xBB);
    }

    #[test]
    fn adc_indirect_y() {
        let mut cpu = CPU {
            pc: 0x0003,
            a: 0x11,
            y: 0x01,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xAA, 0x06, 0xFF, 0xFF, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::IndirectY);

        assert_eq!(cycles, 5);
        assert_eq!(cpu.a, 0xBB);
    }

    #[test]
    fn adc_zeropage() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0x11,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0xDE, 0x01, 0x00])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::ZeroPage);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.a, 0xEF);
    }

    #[test]
    fn adc_zeropage_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            a: 0x11,
            x: 0x02,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0x00, 0xDE, 0x01, 0xAD])
            .expect("Failed to load ram");

        let cycles = adc(&mut cpu, &Addressing::ZeroPageX);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.a, 0xBE);
    }
}
