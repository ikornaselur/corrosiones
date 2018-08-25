//! The insts_test_v5 suite from blargg
//!
//! Downloaded from https://wiki.nesdev.com/w/index.php/Emulator_tests
extern crate corrosiones;

use instr_test_v5::corrosiones::cpu::CPU;
use instr_test_v5::corrosiones::utils::print_rom;

#[test]
fn basics() {
    let mut cpu = CPU::new();
    cpu.load_file(String::from("tests/instr_test_v5/01-basics.nes"))
        .unwrap();

    print_rom(&mut cpu, 16);
    loop {
        match cpu.step() {
            None => break,
            _ => (),
        }
    }
}
