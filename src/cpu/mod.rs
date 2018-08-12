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
            Addressing::Absolute => 0,
            Addressing::AbsoluteX => 0,
            Addressing::AbsoluteY => 0,
            Addressing::Immediate => {
                let byte = self.memory.read(self.pc);
                self.pc += 1;
                byte
            }
            Addressing::Indirect => 0,
            Addressing::IndirectX => 0,
            Addressing::IndirectY => 0,
            Addressing::Relative => 0,
            Addressing::ZeroPage => {
                let address = self.memory.read(self.pc);
                self.pc += 1;
                self.memory.read(u16::from(address))
            }
            Addressing::ZeroPageX => 0,
            Addressing::ZeroPageY => 0,
        }
    }
}
