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
}
