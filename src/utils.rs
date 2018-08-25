use cpu::CPU;

pub fn print_rom(cpu: &CPU, width: usize) {
    let mut count = 0x8000;
    let mut it = cpu.memory.rom.iter();
    while let Some(elm) = it.next() {
        if count % width == 0 {
            print!("\n0x{:04X?} ", count);
        }
        count += 1;
        print!("{:02X?} ", elm);
    }
    println!("");
}
