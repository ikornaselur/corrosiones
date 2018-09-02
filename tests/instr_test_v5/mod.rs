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

#[test]
fn immediate() {
    run_blargg_test("tests/instr_test_v5/03-immediate.nes");
}

#[test]
fn zero_page() {
    run_blargg_test("tests/instr_test_v5/04-zero_page.nes");
}

#[test]
fn zp_xy() {
    run_blargg_test("tests/instr_test_v5/05-zp_xy.nes");
}

#[test]
fn absolute() {
    run_blargg_test("tests/instr_test_v5/06-absolute.nes");
}

#[test]
fn absolute_xy() {
    run_blargg_test("tests/instr_test_v5/07-abs_xy.nes");
}

#[test]
fn indirect_x() {
    run_blargg_test("tests/instr_test_v5/08-ind_x.nes");
}

#[test]
fn indirect_y() {
    run_blargg_test("tests/instr_test_v5/09-ind_y.nes");
}

#[test]
fn branches() {
    run_blargg_test("tests/instr_test_v5/10-branches.nes");
}

#[test]
fn stack() {
    run_blargg_test("tests/instr_test_v5/11-stack.nes");
}

#[test]
fn jump() {
    run_blargg_test("tests/instr_test_v5/12-jmp_jsr.nes");
}

#[test]
fn return_from_subroutine() {
    run_blargg_test("tests/instr_test_v5/13-rts.nes");
}

#[test]
fn return_from_interrupt() {
    run_blargg_test("tests/instr_test_v5/14-rti.nes");
}

#[test]
fn breaks() {
    run_blargg_test("tests/instr_test_v5/15-brk.nes");
}

#[test]
fn special() {
    run_blargg_test("tests/instr_test_v5/16-special.nes");
}
