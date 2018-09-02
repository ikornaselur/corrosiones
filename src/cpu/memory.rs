//! The Memory module
//!
//! Implements the memory as it was in the NES, with write guards for the ROM and mirroring

const RAM_SIZE: usize = 0x0800;
const IO_SIZE: usize = 0x0028;
// const EXPANSION_ROM_SIZE: usize = 0x1980;
const SRAM_SIZE: usize = 0x2000;
const ROM_SIZE: usize = 0x8000;

pub struct Memory {
    ram: Vec<u8>,
    io: Vec<u8>,
    expansion_rom: Vec<u8>,
    pub(crate) sram: Vec<u8>,
    pub(crate) rom: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Memory {
        Memory {
            ram: Vec::new(),
            io: vec![0x00; IO_SIZE],
            expansion_rom: Vec::new(),
            sram: Vec::new(),
            rom: Vec::new(),
        }
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory::default()
    }

    /// Load RAM into memory
    ///
    /// The provided RAM can't exceed the predefined RAM size, while a smaller one will be extended
    /// with 0x00 to fit the it's section in the memory.
    ///
    /// # Arguments
    ///
    /// * `ram` - The RAM to load into memory
    ///
    /// # Example
    ///
    /// ```
    /// let mut memory = corrosiones::cpu::memory::Memory::new();
    ///
    /// memory.load_ram(vec![0xDE, 0xAD, 0xBE, 0xEF]).expect("Failed to load ram");
    /// ```
    pub fn load_ram(&mut self, ram: Vec<u8>) -> Result<(), &'static str> {
        if ram.len() > RAM_SIZE {
            return Err("RAM too big");
        }
        self.ram = ram;
        self.ram.resize(RAM_SIZE, 0x00);

        Ok(())
    }

    pub fn load_sram(&mut self, sram: Vec<u8>) -> Result<(), &'static str> {
        if sram.len() > SRAM_SIZE {
            return Err("RAM too big");
        }
        self.sram = sram;
        self.sram.resize(SRAM_SIZE, 0x00);

        Ok(())
    }

    /// Load ROM into memory
    ///
    /// The provided ROM has to be exactly 0x8000 bytes
    ///
    /// # Arguments
    ///
    /// * `rom` - The ROM to load into memory
    ///
    /// # Example
    ///
    /// ```
    /// let mut memory = corrosiones::cpu::memory::Memory::new();
    ///
    /// memory.load_rom(vec![0xFF; 0x8000]).expect("Failed to load rom");
    /// ```
    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<(), &'static str> {
        if rom.len() != ROM_SIZE {
            return Err("Invalid ROM size");
        }
        self.rom = rom;
        self.rom.resize(ROM_SIZE, 0x00);

        Ok(())
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
    /// memory.load_ram(vec![0xDE, 0xAD, 0xBE, 0xEF]).expect("Failed to load ram");
    ///
    /// assert_eq!(memory.read(0x0001), 0xAD);
    /// assert_eq!(memory.read(0x1001), 0xAD); // Mirrored RAM read
    /// ```
    pub fn read(&self, addr: u16) -> u8 {
        let addr = usize::from(addr);
        let result = match addr {
            0x0000...0x1FFF => self.ram[addr % 0x0800],
            0x2000...0x3FFF => self.io[(addr - 0x2000) % 0x0008],
            0x4000...0x401F => self.io[addr - 0x4000 + 0x0008],
            0x4020...0x5FFF => self.expansion_rom[addr - 0x4020],
            0x6000...0x7FFF => self.sram[addr - 0x6000],
            0x8000...0xFFFF => self.rom[addr - 0x8000],
            _ => panic!("Reading from 0x{:04X?} is unsupported", addr),
        };

        //println!("0x{:04X?}: 0x{:02X?}", addr, result);
        result
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
    /// memory.load_ram(vec![0x00, 0x00]).expect("Failed to load ram");
    ///
    /// memory.write(0x0001, 0xAB);
    ///
    /// assert_eq!(memory.read(0x0001), 0xAB);
    /// ```
    pub(crate) fn write(&mut self, addr: u16, byte: u8) {
        let addr = usize::from(addr);
        // println!("Writing into 0x{:04X?}", addr);
        match addr {
            0x0000...0x1FFF => self.ram[addr % 0x0800] = byte,
            0x2000...0x3FFF => self.io[(addr - 0x2000) % 0x0008] = byte,
            0x4000...0x401F => self.io[addr - 0x4000 + 0x0008] = byte,
            0x6000...0x7FFF => self.sram[addr - 0x6000] = byte,
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
        memory
            .load_ram(vec![0x01, 0x02, 0x03, 0x04])
            .expect("Failed to load ram");

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
