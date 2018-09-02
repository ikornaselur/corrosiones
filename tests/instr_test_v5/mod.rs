//! The insts_test_v5 suite from blargg
//!
//! Downloaded from https://wiki.nesdev.com/w/index.php/Emulator_tests
extern crate corrosiones;

use instr_test_v5::corrosiones::cpu::CPU;
use instr_test_v5::corrosiones::utils::read_blargg_message;

fn run_blargg_test(path: &str) {
    let mut cpu = CPU::new();
    cpu.load_file(String::from(path)).unwrap();

    loop {
        if [0xDE, 0xB0, 0x61]
            == [
                cpu.memory.read(0x6001),
                cpu.memory.read(0x6002),
                cpu.memory.read(0x6003),
            ] {
            match cpu.memory.read(0x6000) {
                0x00 => break, // Passed
                0x80 => {}
                0x81 => panic!("81??"),
                byte => panic!(
                    "\nError code: 0x{:02X?}\n{}\n",
                    byte,
                    read_blargg_message(&mut cpu)
                ),
            }
        }
        cpu.step(false);
    }
}

#[test]
fn basics() {
    run_blargg_test("tests/instr_test_v5/01-basics.nes");
}

#[test]
fn implied() {
    run_blargg_test("tests/instr_test_v5/02-implied.nes");
}
