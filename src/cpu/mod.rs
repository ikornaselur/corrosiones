pub mod addressing;
pub mod flags;
pub mod memory;
pub mod opcodes;
pub mod utils;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use cpu::opcodes::bitwise::and::{aac, and, arr, asr, axs};
use cpu::opcodes::bitwise::or::{eor, ora};
use cpu::opcodes::bitwise::rotate::{rla, rol, ror};
use cpu::opcodes::bitwise::shift::{asl, lsr, slo, sre};
use cpu::opcodes::bitwise::test::bit;
use cpu::opcodes::branch::carry::{bcc, bcs};
use cpu::opcodes::branch::negative::{bmi, bpl};
use cpu::opcodes::branch::overflow::{bvc, bvs};
use cpu::opcodes::branch::zero::{beq, bne};
use cpu::opcodes::jump::jmp::{jmp, jsr};
use cpu::opcodes::jump::ret::{rti, rts};
use cpu::opcodes::math::add::adc;
use cpu::opcodes::math::decrement::{dcp, dec, dex, dey};
use cpu::opcodes::math::increment::{inc, inx, iny};
use cpu::opcodes::math::subtract::sbc;
use cpu::opcodes::registers::clear::{clc, cld, cli, clv};
use cpu::opcodes::registers::compare::{cmp, cpx, cpy};
use cpu::opcodes::registers::set::{sec, sed, sei};
use cpu::opcodes::stack::pull::{pla, plp};
use cpu::opcodes::stack::push::{pha, php};
use cpu::opcodes::storage::load::{lax, lda, ldx, ldy};
use cpu::opcodes::storage::store::{sta, stx, sty};
use cpu::opcodes::storage::transfer::{tax, tay, tsx, txa, txs, tya};
use cpu::opcodes::system::nop;

pub(crate) use cpu::addressing::Addressing;
pub(crate) use cpu::flags::Flags;
pub(crate) use cpu::memory::Memory;

pub struct CPU {
    pub memory: Memory,
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
            sp: 0xFD,
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
            Addressing::AbsoluteX => self
                .read_next_double(progress_pc)
                .wrapping_add(u16::from(self.x)),
            Addressing::AbsoluteY => self
                .read_next_double(progress_pc)
                .wrapping_add(u16::from(self.y)),
            Addressing::IndirectX => {
                let ptr = u16::from(self.read_next_byte(progress_pc) + self.x);
                self.read_double(ptr)
            }
            Addressing::IndirectY => {
                let ptr = self.read_next_byte(progress_pc);
                let lsb = self.memory.read(u16::from(ptr));
                let msb = self.memory.read(u16::from(ptr.wrapping_add(1)));
                let addr = (u16::from(msb) << 8) | u16::from(lsb);
                addr.wrapping_add(u16::from(self.y))
            }
            Addressing::ZeroPage => u16::from(self.read_next_byte(progress_pc)),
            Addressing::ZeroPageX => {
                u16::from(self.read_next_byte(progress_pc).wrapping_add(self.x))
            }
            Addressing::ZeroPageY => {
                u16::from(self.read_next_byte(progress_pc).wrapping_add(self.y))
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

    pub fn update_byte<F>(
        &mut self,
        addressing: &Addressing,
        update_fn: F,
        progress_pc: bool,
    ) -> (u8, Option<bool>)
    where
        F: Fn(u8) -> (u8, Option<bool>),
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
        let (byte, extra) = update_fn(self.memory.read(address));
        self.memory.write(address, byte);
        (byte, extra)
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
        (u16::from(msb) << 8) | u16::from(lsb)
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
        (u16::from(msb) << 8) | u16::from(lsb)
    }

    pub fn offset_pc(&mut self, offset: u8) {
        if offset & 0x80 == 0 {
            self.pc += u16::from(offset);
        } else {
            self.pc -= u16::from(!offset + 1);
        }
    }

    pub fn set_pc(&mut self, address: u16) {
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
        self.memory.load_sram(Vec::new())?;
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

    pub fn step(&mut self, debug: bool) -> Option<u8> {
        if debug {
            println!(
                "{:04X?} A:{:02X?} X:{:02X?} Y:{:02X?} P:{:02X?} SP:{:02X?}",
                self.pc,
                self.a,
                self.x,
                self.y,
                self.flags.as_byte(),
                self.sp
            );
        }
        let byte = self.read_next_byte(true);
        let cycles = match byte {
            // 0x00 => BRK Implied
            0x01 => ora(self, &Addressing::IndirectX),
            0x03 => slo(self, &Addressing::IndirectX),
            0x04 => nop(self, 1, &Addressing::ZeroPage),
            0x05 => ora(self, &Addressing::ZeroPage),
            0x06 => asl(self, &Addressing::ZeroPage),
            0x07 => slo(self, &Addressing::ZeroPage),
            0x08 => php(self),
            0x09 => ora(self, &Addressing::Immediate),
            0x0A => asl(self, &Addressing::Accumulator),
            0x0B => aac(self, &Addressing::Immediate),
            0x0C => nop(self, 2, &Addressing::Absolute),
            0x0D => ora(self, &Addressing::Absolute),
            0x0E => asl(self, &Addressing::Absolute),
            0x0F => slo(self, &Addressing::Absolute),

            0x10 => bpl(self),
            0x11 => ora(self, &Addressing::IndirectY),
            0x13 => slo(self, &Addressing::IndirectY),
            0x14 => nop(self, 1, &Addressing::ZeroPageX),
            0x15 => ora(self, &Addressing::ZeroPageX),
            0x16 => asl(self, &Addressing::ZeroPageX),
            0x17 => slo(self, &Addressing::ZeroPageX),
            0x18 => clc(self),
            0x19 => ora(self, &Addressing::AbsoluteY),
            0x1A => nop(self, 0, &Addressing::Immediate),
            0x1B => slo(self, &Addressing::AbsoluteY),
            0x1C => nop(self, 2, &Addressing::AbsoluteX),
            0x1D => ora(self, &Addressing::AbsoluteX),
            0x1E => asl(self, &Addressing::AbsoluteX),
            0x1F => slo(self, &Addressing::AbsoluteX),

            0x20 => jsr(self, &Addressing::Absolute),
            0x21 => and(self, &Addressing::IndirectX),
            0x23 => rla(self, &Addressing::IndirectX),
            0x24 => bit(self, &Addressing::ZeroPage),
            0x25 => and(self, &Addressing::ZeroPage),
            0x26 => rol(self, &Addressing::ZeroPage),
            0x27 => rla(self, &Addressing::ZeroPage),
            0x28 => plp(self),
            0x29 => and(self, &Addressing::Immediate),
            0x2A => rol(self, &Addressing::Accumulator),
            0x2B => aac(self, &Addressing::Immediate),
            0x2C => bit(self, &Addressing::Absolute),
            0x2D => and(self, &Addressing::Absolute),
            0x2E => rol(self, &Addressing::Absolute),
            0x2F => rla(self, &Addressing::Absolute),

            0x30 => bmi(self),
            0x31 => and(self, &Addressing::IndirectY),
            0x33 => rla(self, &Addressing::IndirectY),
            0x34 => nop(self, 1, &Addressing::ZeroPageX),
            0x35 => and(self, &Addressing::ZeroPageX),
            0x36 => rol(self, &Addressing::ZeroPageX),
            0x37 => rla(self, &Addressing::ZeroPageX),
            0x38 => sec(self),
            0x39 => and(self, &Addressing::AbsoluteY),
            0x3A => nop(self, 0, &Addressing::Immediate),
            0x3B => rla(self, &Addressing::AbsoluteY),
            0x3C => nop(self, 2, &Addressing::AbsoluteX),
            0x3D => and(self, &Addressing::AbsoluteX),
            0x3E => rol(self, &Addressing::AbsoluteX),
            0x3F => rla(self, &Addressing::AbsoluteX),

            0x40 => rti(self),
            0x41 => eor(self, &Addressing::IndirectX),
            0x43 => nop(self, 2, &Addressing::IndirectX),
            0x44 => nop(self, 1, &Addressing::ZeroPage),
            0x45 => eor(self, &Addressing::ZeroPage),
            0x46 => lsr(self, &Addressing::ZeroPage),
            0x47 => sre(self, &Addressing::ZeroPage),
            0x48 => pha(self),
            0x49 => eor(self, &Addressing::Immediate),
            0x4A => lsr(self, &Addressing::Accumulator),
            0x4B => asr(self, &Addressing::Immediate),
            0x4C => jmp(self, &Addressing::Absolute),
            0x4D => eor(self, &Addressing::Absolute),
            0x4E => lsr(self, &Addressing::Absolute),
            0x4F => sre(self, &Addressing::Absolute),

            0x50 => bvc(self),
            0x51 => eor(self, &Addressing::IndirectY),
            0x53 => nop(self, 2, &Addressing::IndirectY),
            0x54 => nop(self, 1, &Addressing::ZeroPageX),
            0x55 => eor(self, &Addressing::ZeroPageX),
            0x56 => lsr(self, &Addressing::ZeroPageX),
            0x57 => sre(self, &Addressing::ZeroPageX),
            0x58 => cli(self),
            0x59 => eor(self, &Addressing::AbsoluteY),
            0x5A => nop(self, 0, &Addressing::Immediate),
            0x5B => sre(self, &Addressing::AbsoluteX),
            0x5C => nop(self, 2, &Addressing::AbsoluteY),
            0x5D => eor(self, &Addressing::AbsoluteX),
            0x5E => lsr(self, &Addressing::AbsoluteX),
            0x5F => sre(self, &Addressing::AbsoluteX),

            0x60 => rts(self),
            0x61 => adc(self, &Addressing::IndirectX),
            0x64 => nop(self, 1, &Addressing::ZeroPage),
            0x65 => adc(self, &Addressing::ZeroPage),
            0x66 => ror(self, &Addressing::ZeroPage),
            0x68 => pla(self),
            0x69 => adc(self, &Addressing::Immediate),
            0x6A => ror(self, &Addressing::Accumulator),
            0x6B => arr(self, &Addressing::Immediate),
            0x6C => jmp(self, &Addressing::Indirect),
            0x6D => adc(self, &Addressing::Absolute),
            0x6E => ror(self, &Addressing::Absolute),

            0x70 => bvs(self),
            0x71 => adc(self, &Addressing::IndirectY),
            0x74 => nop(self, 1, &Addressing::ZeroPageX),
            0x75 => adc(self, &Addressing::ZeroPageX),
            0x76 => ror(self, &Addressing::ZeroPageX),
            0x78 => sei(self),
            0x79 => adc(self, &Addressing::AbsoluteY),
            0x7A => nop(self, 0, &Addressing::Immediate),
            0x7C => nop(self, 2, &Addressing::AbsoluteX),
            0x7D => adc(self, &Addressing::AbsoluteX),
            0x7E => ror(self, &Addressing::AbsoluteX),

            0x80 => nop(self, 1, &Addressing::Immediate),
            0x81 => sta(self, &Addressing::IndirectX),
            0x82 => nop(self, 1, &Addressing::Immediate),
            0x84 => sty(self, &Addressing::ZeroPage),
            0x85 => sta(self, &Addressing::ZeroPage),
            0x86 => stx(self, &Addressing::ZeroPage),
            0x88 => dey(self),
            0x89 => nop(self, 1, &Addressing::Immediate),
            0x8A => txa(self),
            0x8C => sty(self, &Addressing::Absolute),
            0x8D => sta(self, &Addressing::Absolute),
            0x8E => stx(self, &Addressing::Absolute),

            0x90 => bcc(self),
            0x91 => sta(self, &Addressing::IndirectY),
            0x94 => sty(self, &Addressing::ZeroPageX),
            0x95 => sta(self, &Addressing::ZeroPageX),
            0x96 => stx(self, &Addressing::ZeroPageY),
            0x98 => tya(self),
            0x99 => sta(self, &Addressing::AbsoluteY),
            0x9A => txs(self),
            0x9D => sta(self, &Addressing::AbsoluteX),

            0xA0 => ldy(self, &Addressing::Immediate),
            0xA1 => lda(self, &Addressing::IndirectX),
            0xA2 => ldx(self, &Addressing::Immediate),
            0xA3 => lax(self, &Addressing::IndirectX),
            0xA4 => ldy(self, &Addressing::ZeroPage),
            0xA5 => lda(self, &Addressing::ZeroPage),
            0xA6 => ldx(self, &Addressing::ZeroPage),
            0xA7 => lax(self, &Addressing::ZeroPage),
            0xA8 => tay(self),
            0xA9 => lda(self, &Addressing::Immediate),
            0xAA => tax(self),
            0xAB => lax(self, &Addressing::Immediate),
            0xAC => ldy(self, &Addressing::Absolute),
            0xAD => lda(self, &Addressing::Absolute),
            0xAE => ldx(self, &Addressing::Absolute),
            0xAF => lax(self, &Addressing::Absolute),

            0xB0 => bcs(self),
            0xB1 => lda(self, &Addressing::IndirectY),
            0xB3 => lax(self, &Addressing::IndirectY),
            0xB4 => ldy(self, &Addressing::ZeroPageX),
            0xB5 => lda(self, &Addressing::ZeroPageX),
            0xB6 => ldx(self, &Addressing::ZeroPageY),
            0xB7 => lax(self, &Addressing::ZeroPageY),
            0xB8 => clv(self),
            0xB9 => lda(self, &Addressing::AbsoluteY),
            0xBA => tsx(self),
            0xBC => ldy(self, &Addressing::AbsoluteX),
            0xBD => lda(self, &Addressing::AbsoluteX),
            0xBE => ldx(self, &Addressing::AbsoluteY),
            0xBF => lax(self, &Addressing::AbsoluteY),

            0xC0 => cpy(self, &Addressing::Immediate),
            0xC1 => cmp(self, &Addressing::IndirectX),
            0xC2 => nop(self, 1, &Addressing::Immediate),
            0xC3 => dcp(self, &Addressing::IndirectX),
            0xC4 => cpy(self, &Addressing::ZeroPage),
            0xC5 => cmp(self, &Addressing::ZeroPage),
            0xC6 => dec(self, &Addressing::ZeroPage),
            0xC7 => dcp(self, &Addressing::ZeroPage),
            0xC8 => iny(self),
            0xC9 => cmp(self, &Addressing::Immediate),
            0xCA => dex(self),
            0xCB => axs(self, &Addressing::Immediate),
            0xCC => cpy(self, &Addressing::Absolute),
            0xCD => cmp(self, &Addressing::Absolute),
            0xCE => dec(self, &Addressing::Absolute),
            0xCF => dcp(self, &Addressing::Absolute),

            0xD0 => bne(self),
            0xD1 => cmp(self, &Addressing::IndirectY),
            0xD3 => dcp(self, &Addressing::IndirectY),
            0xD4 => nop(self, 1, &Addressing::ZeroPageX),
            0xD5 => cmp(self, &Addressing::ZeroPageX),
            0xD6 => dec(self, &Addressing::ZeroPageX),
            0xD7 => dcp(self, &Addressing::ZeroPageX),
            0xD8 => cld(self),
            0xD9 => cmp(self, &Addressing::AbsoluteY),
            0xDA => nop(self, 0, &Addressing::Immediate),
            0xDB => dcp(self, &Addressing::AbsoluteY),
            0xDC => nop(self, 2, &Addressing::AbsoluteX),
            0xDD => cmp(self, &Addressing::AbsoluteX),
            0xDE => dec(self, &Addressing::AbsoluteX),
            0xDF => dcp(self, &Addressing::AbsoluteX),

            0xE0 => cpx(self, &Addressing::Immediate),
            0xE1 => sbc(self, &Addressing::IndirectX),
            0xE2 => nop(self, 1, &Addressing::Immediate),
            0xE4 => cpx(self, &Addressing::ZeroPage),
            0xE5 => sbc(self, &Addressing::ZeroPage),
            0xE6 => inc(self, &Addressing::ZeroPage),
            0xE8 => inx(self),
            0xE9 => sbc(self, &Addressing::Immediate),
            0xEA => nop(self, 0, &Addressing::Immediate),
            0xEB => sbc(self, &Addressing::Immediate),
            0xEC => cpx(self, &Addressing::Absolute),
            0xED => sbc(self, &Addressing::Absolute),
            0xEE => inc(self, &Addressing::Absolute),

            0xF0 => beq(self),
            0xF1 => sbc(self, &Addressing::IndirectY),
            0xF4 => nop(self, 1, &Addressing::ZeroPageX),
            0xF5 => sbc(self, &Addressing::ZeroPageX),
            0xF6 => inc(self, &Addressing::ZeroPageX),
            0xF8 => sed(self),
            0xF9 => sbc(self, &Addressing::AbsoluteY),
            0xFA => nop(self, 0, &Addressing::Immediate),
            0xFC => nop(self, 2, &Addressing::AbsoluteX),
            0xFD => sbc(self, &Addressing::AbsoluteX),
            0xFE => inc(self, &Addressing::AbsoluteX),

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

        assert_eq!(cpu.sp, 0xFB);
        assert_eq!(cpu.raw_read_byte(0x01FD), 0xAD);
        assert_eq!(cpu.raw_read_byte(0x01FC), 0xDE);
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
        let mut cpu = CPU {
            sp: 0xFF,
            ..CPU::default()
        };
        cpu.memory.load_ram(Vec::new()).expect("Failed to load ram");
        cpu.raw_write_byte(0x0100, 0xFF);

        let result = cpu.pop_stack();

        assert_eq!(cpu.sp, 0x00);
        assert_eq!(result, 0xFF);
    }

    #[test]
    fn offset_pc_by_max_negative() {
        let mut cpu = CPU {
            pc: 1000,
            ..CPU::default()
        };

        cpu.offset_pc(0x80);

        assert_eq!(cpu.pc, 1000 - 128);
    }

    #[test]
    fn offset_pc_by_negative_one() {
        let mut cpu = CPU {
            pc: 1000,
            ..CPU::default()
        };

        cpu.offset_pc(0xFF);

        assert_eq!(cpu.pc, 1000 - 1);
    }

    #[test]
    fn offset_pc_by_zero() {
        let mut cpu = CPU {
            pc: 1000,
            ..CPU::default()
        };

        cpu.offset_pc(0x00);

        assert_eq!(cpu.pc, 1000);
    }

    #[test]
    fn offset_pc_by_positive_one() {
        let mut cpu = CPU {
            pc: 1000,
            ..CPU::default()
        };

        cpu.offset_pc(0x01);

        assert_eq!(cpu.pc, 1000 + 1);
    }
    #[test]
    fn offset_pc_by_max_positive() {
        let mut cpu = CPU {
            pc: 1000,
            ..CPU::default()
        };

        cpu.offset_pc(0x7F);

        assert_eq!(cpu.pc, 1000 + 127);
    }
}
