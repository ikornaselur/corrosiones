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
        let address = match addressing {
            Addressing::Immediate => {
                // Return immediately the nest byte on immediate
                return self.read_next_byte();
            }
            Addressing::Absolute => self.read_next_double(),
            Addressing::AbsoluteX => self.read_next_double() + u16::from(self.x),
            Addressing::AbsoluteY => self.read_next_double() + u16::from(self.y),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte() + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte());
                self.read_double(ptr) + u16::from(self.y)
            }
            Addressing::ZeroPage => self.read_next_byte() as u16,
            Addressing::ZeroPageX => self.read_next_byte().wrapping_add(self.x) as u16,
            _ => panic!("read_byte doesn't support {:?} addressing", addressing),
        };
        self.memory.read(address)
    }

    pub fn write_byte(&mut self, byte: u8, addressing: Addressing) {
        let address = match addressing {
            Addressing::Absolute => self.read_next_double(),
            Addressing::AbsoluteX => self.read_next_double() + u16::from(self.x),
            Addressing::AbsoluteY => self.read_next_double() + u16::from(self.y),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte() + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte());
                self.read_double(ptr) + u16::from(self.y)
            }
            Addressing::ZeroPage => self.read_next_byte() as u16,
            Addressing::ZeroPageX => self.read_next_byte().wrapping_add(self.x) as u16,
            Addressing::ZeroPageY => self.read_next_byte().wrapping_add(self.y) as u16,
            _ => panic!("write_byte doesn't support {:?} addressing", addressing),
        };
        self.memory.write(address, byte);
    }

    pub fn update_byte<F>(&mut self, addressing: Addressing, update_fn: F) -> u8
    where
        F: Fn(u8) -> u8,
    {
        let address = match addressing {
            Addressing::Absolute => self.read_next_double(),
            Addressing::AbsoluteX => self.read_next_double() + u16::from(self.x),
            Addressing::AbsoluteY => self.read_next_double() + u16::from(self.y),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte() + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte());
                self.read_double(ptr) + u16::from(self.y)
            }
            Addressing::ZeroPage => self.read_next_byte() as u16,
            Addressing::ZeroPageX => self.read_next_byte().wrapping_add(self.x) as u16,
            _ => panic!("update_byte doesn't support {:?} addressing", addressing),
        };
        let byte = update_fn(self.memory.read(address));
        self.memory.write(address, byte);
        byte
    }

    fn read_next_byte(&mut self) -> u8 {
        let byte = self.memory.read(self.pc);
        self.pc += 1;
        byte
    }

    fn read_next_double(&mut self) -> u16 {
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
