//! The Memory module
//!
//! Implements the memory as it was in the NES, with write guards for the ROM and mirroring

const RAM_SIZE: usize = 0x0800;
const IO_SIZE: usize = 0x0028;
const EXPANSION_ROM_SIZE: usize = 0x1980;
const SRAM_SIZE: usize = 0x2000;
const ROM_SIZE: usize = 0x2000;

pub struct Memory {
    ram: Vec<u8>,
    io: Vec<u8>,
    expansion_rom: Vec<u8>,
    sram: Vec<u8>,
    rom: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: Vec::new(),
            io: vec![0x00; IO_SIZE],
            expansion_rom: Vec::new(),
            sram: Vec::new(),
            rom: Vec::new(),
        }
    }

    /// Load RAM into memory
    ///
    /// The provided RAM can't exceed the predefined RAM size, while a smaller one will be extended
    /// with 0x00 to fit the it's section in the memory.
    ///
    /// # Arguments
    ///
    /// * `ram` - The ram to load into memory
    ///
    /// # Example
    ///
    /// ```
    /// let mut memory = corrosiones::cpu::memory::Memory::new();
    ///
    /// memory.load_ram(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    /// ```
    pub fn load_ram(&mut self, ram: Vec<u8>) {
        if ram.len() > RAM_SIZE {
            panic!(
                "Ram too long, can't exceed 0x{:04X?}. Was: {:04X?}",
                RAM_SIZE,
                ram.len()
            )
        }
        self.ram = ram;
        self.ram.resize(RAM_SIZE, 0x00);
    }

    /// Read from the memory
    ///
    /// Returns the correct byte from memory, after taking in account any mirroring
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read from
    ///
    /// # Example
    ///
    /// ```
    /// let mut memory = corrosiones::cpu::memory::Memory::new();
    ///
    /// memory.load_ram(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    ///
    /// assert_eq!(memory.read(0x0001), 0xAD);
    /// assert_eq!(memory.read(0x1001), 0xAD); // Mirrored RAM read
    /// ```
    pub fn read(&self, addr: u16) -> u8 {
        let addr = usize::from(addr);
        match addr {
            0x0000...0x07FF => self.ram[addr],
            0x0800...0x0FFF => self.ram[addr - 0x0800],
            0x1000...0x17FF => self.ram[addr - 0x1000],
            0x1800...0x1FFF => self.ram[addr - 0x1800],
            0x2000...0x3FFF => self.io[(addr - 0x2000) % 0x0008],
            0x4000...0x401F => self.io[addr - 0x4000 + 0x0008],
            0x4020...0x5FFF => self.expansion_rom[addr - 0x4020],
            0x6000...0x7FFF => self.sram[addr - 0x6000],
            0x8000...0xFFFF => self.rom[addr - 0x8000],
            _ => panic!("Reading from 0x{:04X?} is unsupported", addr),
        }
    }

    /// Write to the memory
    ///
    /// # Panics
    ///
    /// Panics if trying to write to a read only part of the memory
    ///
    /// # Example
    ///
    /// ```
    /// let mut memory = corrosiones::cpu::memory::Memory::new();
    ///
    /// memory.load_ram(vec![0x00, 0x00]);
    ///
    /// memory.write(0x0001, 0xAB);
    ///
    /// assert_eq!(memory.read(0x0001), 0xAB);
    /// ```
    pub fn write(&mut self, addr: u16, byte: u8) {
        let addr = usize::from(addr);
        match addr {
            0x0000...0x07FF => self.ram[addr] = byte,
            0x0800...0x0FFF => self.ram[addr - 0x0800] = byte,
            0x1000...0x17FF => self.ram[addr - 0x1000] = byte,
            0x1800...0x1FFF => self.ram[addr - 0x1800] = byte,
            _ => panic!("Unable to write to 0x{:04X?}", addr),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_ram() {
        let mut memory = Memory::new();
        memory.load_ram(vec![0x01, 0x02, 0x03, 0x04]);

        assert_eq!(memory.ram[0], 0x01);
        assert_eq!(memory.ram[1], 0x02);
        assert_eq!(memory.ram[2], 0x03);
        assert_eq!(memory.ram[3], 0x04);
    }

    #[test]
    fn read_from_ram() {
        let mut memory = Memory::new();
        memory.ram = vec![0x01, 0x02, 0x03, 0x04];

        assert_eq!(memory.read(0x0000), 0x01);
        assert_eq!(memory.read(0x0801), 0x02);
        assert_eq!(memory.read(0x1002), 0x03);
        assert_eq!(memory.read(0x1803), 0x04);
    }

    #[test]
    fn read_from_lower_io() {
        let mut memory = Memory::new();
        memory.io = vec![0x01, 0x02, 0x03, 0x04];

        assert_eq!(memory.read(0x2000), 0x01);
        assert_eq!(memory.read(0x2008), 0x01);
        assert_eq!(memory.read(0x2010), 0x01);
        assert_eq!(memory.read(0x2018), 0x01);
        assert_eq!(memory.read(0x3FF8), 0x01);
    }

    #[test]
    fn read_from_upper_io() {
        let mut memory = Memory::new();
        memory.io = vec![0x00; 0x28];
        memory.io[0x08] = 0xDE;
        memory.io[0x27] = 0xAD;

        assert_eq!(memory.read(0x4000), 0xDE);
        assert_eq!(memory.read(0x401F), 0xAD);
    }

    #[test]
    fn read_from_expansion_rom() {
        let mut memory = Memory::new();
        memory.expansion_rom = vec![0x01, 0x02];

        assert_eq!(memory.read(0x4020), 0x01);
        assert_eq!(memory.read(0x4021), 0x02);
    }

    #[test]
    fn read_from_sram() {
        let mut memory = Memory::new();
        memory.sram = vec![0x01, 0x02];

        assert_eq!(memory.read(0x6000), 0x01);
        assert_eq!(memory.read(0x6001), 0x02);
    }

    #[test]
    fn read_from_rom() {
        let mut memory = Memory::new();
        memory.rom = vec![0x01, 0x02];

        assert_eq!(memory.read(0x8000), 0x01);
        assert_eq!(memory.read(0x8001), 0x02);
    }
}
