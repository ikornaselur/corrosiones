pub mod addressing;
pub mod flags;
pub mod memory;
pub mod opcodes;
pub mod utils;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use cpu::opcodes::{jump, registers, storage};

pub(crate) use cpu::addressing::Addressing;
pub(crate) use cpu::flags::Flags;
pub(crate) use cpu::memory::Memory;

pub struct CPU {
    pub(crate) memory: Memory,
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
            sp: 0xFF,
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

    pub fn read_byte(&mut self, addressing: &Addressing, progress_pc: bool) -> u8 {
        let address = match addressing {
            Addressing::Immediate => {
                // Return immediately the next byte on immediate
                return self.read_next_byte(progress_pc);
            }
            Addressing::Accumulator => {
                // Return immediately the accumulator
                return self.a;
            }
            Addressing::Absolute => self.read_next_double(progress_pc),
            Addressing::AbsoluteX => self.read_next_double(progress_pc) + u16::from(self.x),
            Addressing::AbsoluteY => self.read_next_double(progress_pc) + u16::from(self.y),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte(progress_pc) + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte(progress_pc));
                self.read_double(ptr) + u16::from(self.y)
            }
            Addressing::ZeroPage => u16::from(self.read_next_byte(progress_pc)),
            Addressing::ZeroPageX => {
                u16::from(self.read_next_byte(progress_pc).wrapping_add(self.x))
            }
            _ => panic!("read_byte doesn't support {:?} addressing", addressing),
        };
        self.memory.read(address)
    }

    pub fn write_byte(&mut self, addressing: &Addressing, byte: u8, progress_pc: bool) {
        let address = match addressing {
            Addressing::Accumulator => {
                // Return immediately after setting the accumulator
                self.a = byte;
                return ();
            }
            Addressing::Absolute => self.read_next_double(true),
            Addressing::AbsoluteX => self.read_next_double(true) + u16::from(self.x),
            Addressing::AbsoluteY => self.read_next_double(true) + u16::from(self.y),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte(progress_pc) + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte(progress_pc));
                self.read_double(ptr) + u16::from(self.y)
            }
            Addressing::ZeroPage => u16::from(self.read_next_byte(progress_pc)),
            Addressing::ZeroPageX => {
                u16::from(self.read_next_byte(progress_pc).wrapping_add(self.x))
            }
            Addressing::ZeroPageY => {
                u16::from(self.read_next_byte(progress_pc).wrapping_add(self.y))
            }
            _ => panic!("write_byte doesn't support {:?} addressing", addressing),
        };
        self.memory.write(address, byte);
    }

    pub fn update_byte<F>(&mut self, addressing: &Addressing, update_fn: F, progress_pc: bool) -> u8
    where
        F: Fn(u8) -> u8,
    {
        let address = match addressing {
            Addressing::Absolute => self.read_next_double(progress_pc),
            Addressing::AbsoluteX => self.read_next_double(progress_pc) + u16::from(self.x),
            Addressing::AbsoluteY => self.read_next_double(progress_pc) + u16::from(self.y),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte(progress_pc) + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = u16::from(self.read_next_byte(progress_pc));
                self.read_double(ptr) + u16::from(self.y)
            }
            Addressing::ZeroPage => u16::from(self.read_next_byte(progress_pc)),
            Addressing::ZeroPageX => {
                u16::from(self.read_next_byte(progress_pc).wrapping_add(self.x))
            }
            _ => panic!("update_byte doesn't support {:?} addressing", addressing),
        };
        let byte = update_fn(self.memory.read(address));
        self.memory.write(address, byte);
        byte
    }

    fn read_next_byte(&mut self, progress_pc: bool) -> u8 {
        let byte = self.memory.read(self.pc);
        if progress_pc {
            self.pc += 1;
        }
        byte
    }

    fn read_next_double(&mut self, progress_pc: bool) -> u16 {
        let lsb = self.memory.read(self.pc);
        let msb = self.memory.read(self.pc + 1);
        if progress_pc {
            self.pc += 2;
        }
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

    pub fn offset_pc(&mut self, offset: u16) {
        self.pc += offset;
    }

    fn set_pc(&mut self, address: u16) {
        self.pc = address;
    }

    /// Push to the stack
    fn push_stack(&mut self, byte: u8) {
        self.memory.write(u16::from(self.sp) + 0x0100, byte);
        self.sp = self.sp.wrapping_sub(1);
    }

    /// Pop from the stack
    fn pop_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory.read(u16::from(self.sp) + 0x0100)
    }

    pub fn load_file(&mut self, filename: String) -> Result<(), Box<Error>> {
        let mut f = File::open(filename)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        self.process_file(&buffer[..])?;
        self.memory.load_ram(Vec::new())?;
        self.reset_vector();

        Ok(())
    }

    fn process_file(&mut self, buffer: &[u8]) -> Result<(), &'static str> {
        if buffer[0..=3] != [b'N', b'E', b'S', 0x1A] {
            return Err("Invalid magic header");
        }
        let rom_control_byte1 = buffer[6];
        let rom_control_byte2 = buffer[7];

        let mapper = (rom_control_byte1 & 0b1111_0000) >> 4 | (rom_control_byte2 & 0b1111_0000);

        match mapper {
            0 => nrom(self, buffer)?,
            _ => return Err("Unsupported mapper"),
        }

        Ok(())
    }

    /// Jump to the reset vector
    fn reset_vector(&mut self) {
        let address = self.read_double(0xFFFC);
        self.pc = address;
    }

    pub fn step(&mut self) -> Option<u8> {
        println!(
            "A: 0x{:02X?} X: 0x{:02X?} Y: 0x{:02X?} SP: 0x{:02X?} PC: 0x{:04X?}",
            self.a, self.x, self.y, self.sp, self.pc
        );
        let byte = self.read_next_byte(true);
        let cycles = match byte {
            0x4C => jump::jmp::jmp(self, &Addressing::Absolute),
            0x78 => registers::set::sei(self),
            0x8D => storage::store::sta(self, &Addressing::Absolute),
            0xA9 => storage::load::lda(self, &Addressing::Immediate),
            _ => panic!("Unknown opcode: 0x{:02X?}", byte),
        };

        Some(cycles)
    }
}

pub fn nrom(cpu: &mut CPU, buffer: &[u8]) -> Result<(), &'static str> {
    let trainer = buffer[6] & 0b0000_0100 > 0;
    let bank_offset = if trainer { 16 + 512 } else { 16 };
    match buffer[4] {
        1 => {
            let mut bank = Vec::from(&buffer[bank_offset..(bank_offset + 0x4000)]);
            bank.extend(&buffer[bank_offset..(bank_offset + 0x4000)]);
            cpu.memory.load_rom(bank)?;
        }
        2 => cpu
            .memory
            .load_rom(Vec::from(&buffer[bank_offset..(bank_offset + 0x8000)]))?,
        _ => {
            return Err("NROM only supports 1 or 2 PRG ROM banks");
        }
    }
    Ok(())
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
        cpu.memory
            .load_ram(vec![0xFF, 0xFF, 0xAA, 0xFF])
            .expect("Failed to load ram");

        let byte = cpu.read_next_byte(true);

        assert_eq!(byte, 0xAA);
        assert_eq!(cpu.pc, 0x0003);
    }

    #[test]
    fn pushing_to_the_stack() {
        let mut cpu = CPU::new();
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");

        cpu.push_stack(0xAD);
        cpu.push_stack(0xDE);

        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.raw_read_byte(0x01FF), 0xAD);
        assert_eq!(cpu.raw_read_byte(0x01FE), 0xDE);
    }

    #[test]
    fn pushing_to_the_stack_wraps_the_stack_pointer() {
        let mut cpu = CPU {
            sp: 0x00,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");

        cpu.push_stack(0xAD);
        cpu.push_stack(0xDE);

        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.raw_read_byte(0x0100), 0xAD);
        assert_eq!(cpu.raw_read_byte(0x01FF), 0xDE);
    }

    #[test]
    fn reading_from_the_stack() {
        let mut cpu = CPU {
            sp: 0xFD,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x01FE, 0xDE);
        cpu.raw_write_byte(0x01FF, 0xAD);

        let first = cpu.pop_stack();
        let second = cpu.pop_stack();

        assert_eq!(cpu.sp, 0xFF);
        assert_eq!(first, 0xDE);
        assert_eq!(second, 0xAD);
    }

    #[test]
    fn reading_from_the_stack_wraps_the_stack_pointer() {
        let mut cpu = CPU::new();
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x0100, 0xFF);

        let result = cpu.pop_stack();

        assert_eq!(cpu.sp, 0x00);
        assert_eq!(result, 0xFF);
    }
}
