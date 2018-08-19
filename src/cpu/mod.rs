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
                let address = self.read_next_double();
                self.memory.read(address)
            }
            Addressing::AbsoluteX => {
                let address = self.read_next_double() + u16::from(self.x);
                self.memory.read(address)
            }
            Addressing::AbsoluteY => {
                let address = self.read_next_double() + u16::from(self.y);
                self.memory.read(address)
            }
            Addressing::Immediate => self.read_next_byte(),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte() + self.x);

                let address = self.read_double(ptr);

                self.memory.read(address)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte());

                let address = self.read_double(ptr) + u16::from(self.y);

                self.memory.read(address)
            }
            Addressing::ZeroPage => {
                let address = self.read_next_byte() as u16;
                self.memory.read(address)
            }
            Addressing::ZeroPageX => {
                let address = self.read_next_byte().wrapping_add(self.x) as u16;
                self.memory.read(address)
            }
            _ => panic!("read_byte doesn't support {:?} addressing", addressing),
        }
    }

    pub fn write_byte(&mut self, byte: u8, addressing: Addressing) {
        match addressing {
            Addressing::Absolute => {
                let address = self.read_next_double();
                self.memory.write(address, byte);
            }
            Addressing::AbsoluteX => {
                let address = self.read_next_double() + u16::from(self.x);
                self.memory.write(address, byte);
            }
            Addressing::AbsoluteY => {
                let address = self.read_next_double() + u16::from(self.y);
                self.memory.write(address, byte);
            }
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte() + self.x);

                let address = self.read_double(ptr);

                self.memory.write(address, byte);
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte());

                let address = self.read_double(ptr) + u16::from(self.y);

                self.memory.write(address, byte);
            }
            Addressing::ZeroPage => {
                let address = self.read_next_byte() as u16;

                self.memory.write(address, byte);
            }
            Addressing::ZeroPageX => {
                let address = self.read_next_byte().wrapping_add(self.x) as u16;

                self.memory.write(address, byte);
            }
            Addressing::ZeroPageY => {
                let address = self.read_next_byte().wrapping_add(self.y) as u16;

                self.memory.write(address, byte);
            }
            _ => panic!("write_byte doesn't support {:?} addressing", addressing),
        };
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let byte = self.memory.read(self.pc);
        self.pc += 1;
        byte
    }

    pub fn read_next_double(&mut self) -> u16 {
        let lsb = self.read_next_byte();
        let msb = self.read_next_byte();
        (u16::from(msb) << 8) + u16::from(lsb)
    }

    /// Read a byte from an address
    pub fn raw_read_byte(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    /// Write a byte to a memory address
    pub fn raw_write_byte(&mut self, address: u16, byte: u8) {
        self.memory.write(address, byte);
    }

    /// Read a double from an address
    ///
    /// Reads two bytes and combines them in a 16-bit double in little endian
    pub fn read_double(&self, address: u16) -> u16 {
        let lsb = self.memory.read(address);
        let msb = self.memory.read(address + 1);
        (u16::from(msb) << 8) + u16::from(lsb)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_next_byte() {
        let mut cpu = CPU {
            pc: 0x0002,
            ..CPU::default()
        };
        cpu.memory.load_ram(vec![0xFF, 0xFF, 0xAA, 0xFF]);

        let byte = cpu.read_next_byte();

        assert_eq!(byte, 0xAA);
        assert_eq!(cpu.pc, 0x0003);
    }
}
