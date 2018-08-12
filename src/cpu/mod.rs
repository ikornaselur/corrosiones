pub mod addressing;
pub mod flags;
pub mod memory;
pub mod opcodes;

pub(crate) use cpu::addressing::Addressing;
pub(crate) use cpu::flags::Flags;
pub(crate) use cpu::memory::Memory;

pub struct CPU {
    memory: Memory,
    flags: Flags,
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            memory: Memory::new(),
            flags: Flags::new(),
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
        }
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU::default()
    }

    pub fn read_byte(&mut self, addressing: Addressing) -> u8 {
        match addressing {
            Addressing::Absolute => {
                let lsb = self.memory.read(self.pc);
                self.pc += 1;

                let msb = self.memory.read(self.pc);
                self.pc += 1;

                let address = (u16::from(msb) << 8) + u16::from(lsb);
                self.memory.read(address)
            }
            Addressing::AbsoluteX => {
                let lsb = self.memory.read(self.pc);
                self.pc += 1;

                let msb = self.memory.read(self.pc);
                self.pc += 1;

                let mut address = (u16::from(msb) << 8) + u16::from(lsb) + u16::from(self.x);

                self.memory.read(address)
            }
            Addressing::AbsoluteY => {
                let lsb = self.memory.read(self.pc);
                self.pc += 1;

                let msb = self.memory.read(self.pc);
                self.pc += 1;

                let mut address = (u16::from(msb) << 8) + u16::from(lsb) + u16::from(self.y);

                self.memory.read(address)
            }
            Addressing::Immediate => {
                let byte = self.memory.read(self.pc);
                self.pc += 1;
                byte
            }
            Addressing::Indirect => 0,
            Addressing::IndirectX => 0,
            Addressing::IndirectY => 0,
            Addressing::Relative => {
                let offset = self.memory.read(self.pc);
                self.pc += 1;
                self.pc = self.pc.wrapping_add((offset as i8) as u16);
                offset
            }
            Addressing::ZeroPage => {
                let address = self.memory.read(self.pc);
                self.pc += 1;
                self.memory.read(u16::from(address))
            }
            Addressing::ZeroPageX => {
                let address = self.memory.read(self.pc);
                self.pc += 1;
                let address = address.wrapping_add(self.x);
                self.memory.read(u16::from(address))
            }
            Addressing::ZeroPageY => {
                let address = self.memory.read(self.pc);
                self.pc += 1;
                let address = address.wrapping_add(self.y);
                self.memory.read(u16::from(address))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_byte_absolute() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x05, 0x00, 0xFF, 0xAA]);

        let byte = cpu.read_byte(Addressing::Absolute);

        assert_eq!(byte, 0xAA);
        assert_eq!(cpu.pc, 0x0004);
    }

    #[test]
    fn read_byte_absolute_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 1,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x04, 0x00, 0xFF, 0xAA]);

        let byte = cpu.read_byte(Addressing::AbsoluteX);

        assert_eq!(byte, 0xAA);
        assert_eq!(cpu.pc, 0x0004);
    }

    #[test]
    fn read_byte_absolute_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            y: 1,
            ..CPU::default()
        };
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0x04, 0x00, 0xFF, 0xAA]);

        let byte = cpu.read_byte(Addressing::AbsoluteY);

        assert_eq!(byte, 0xAA);
        assert_eq!(cpu.pc, 0x0004);
    }

    #[test]
    fn read_byte_immediate() {
        let mut cpu = CPU {
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xDE, 0xAD, 0xBE, 0xEF]);

        let byte = cpu.read_byte(Addressing::Immediate);

        assert_eq!(byte, 0xAD);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn read_byte_indirect() {
        let mut cpu = CPU {
            pc: 0x0001,
            ..CPU::default()
        };
    }

    #[test]
    fn read_byte_indirect_x() {}

    #[test]
    fn read_byte_indirect_y() {}

    #[test]
    fn read_byte_relative() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0x00, 0x00, -2i8 as u8, 0x00]);

        cpu.read_byte(Addressing::Relative);

        assert_eq!(cpu.pc, 0x0001);
    }

    #[test]
    fn read_byte_zero_page() {
        let mut cpu = CPU {
            pc: 0x0001,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xDE, 0x03, 0xBE, 0xEF]);

        let byte = cpu.read_byte(Addressing::ZeroPage);

        assert_eq!(byte, 0xEF);
        assert_eq!(cpu.pc, 0x0002);
    }

    #[test]
    fn read_byte_zero_page_x() {
        let mut cpu = CPU {
            pc: 0x0002,
            x: 0xFF,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xDE, 0xAD, 0x04, 0xEF]);

        let byte = cpu.read_byte(Addressing::ZeroPageX);

        assert_eq!(byte, 0xEF);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn read_byte_zero_page_y() {
        let mut cpu = CPU {
            pc: 0x0002,
            y: 0xFF,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xDE, 0xAD, 0x04, 0xEF]);

        let byte = cpu.read_byte(Addressing::ZeroPageY);

        assert_eq!(byte, 0xEF);
        assert_eq!(cpu.pc, 0x0003);
    }
}
