use cpu::CPU;

pub fn print_rom(cpu: &CPU, width: usize) {
    let mut count = 0x8000;
    let it = cpu.memory.rom.iter();
    for elm in it {
        if count % width == 0 {
            print!("\n0x{:04X?} ", count);
        }
        count += 1;
        print!("{:02X?} ", elm);
    }
    println!();
}

pub fn read_blargg_message(cpu: &mut CPU) -> String {
    let mut bytes: Vec<u8> = vec![];

    let mut index = 0x6004;

    loop {
        let byte = cpu.memory.read(index);
        if byte == 0x00 {
            break;
        }
        bytes.push(byte);
        index += 1;
    }
    String::from_utf8(bytes).unwrap()
}
