extern crate elf;

use std::path::PathBuf;
use std::env;
use std::io::{BufWriter, Write};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        panic!("not enough argument");
    }
    let path = PathBuf::from(&args[0]);
    let base_addr = u64::from_str_radix(&args[1], 16);

    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let mut mapping_sections :Vec<_> = file.sections
                            .iter()
                            .filter(|&section| section.shdr.shtype == elf::types::SHT_PROGBITS)
                            .collect();

    mapping_sections.sort_by(
            |&sec_a, &sec_b| sec_a.shdr.addr.cmp(&sec_b.shdr.addr));

    let mut current_addr = match base_addr {
            Ok(n) => n,
            Err(err) => panic!("error {}", err),
        };

    let f = fs::File::create(
            format!("{}.hex", path.file_name().unwrap().to_str().unwrap())
            ).unwrap();
    let mut writer = BufWriter::new(f);

    for section in mapping_sections
    {
        if section.shdr.addr < current_addr
        {
            continue;
        }
        println!("{}", section);
        while current_addr < section.shdr.addr
        {
            current_addr += 1;
            //println!("0x{:x} 00", current_addr);
            write!(writer, "00\n").unwrap();
        }
        for byte in &section.data
        {
            current_addr += 1;
            //println!("0x{:x} {:02x}", current_addr, byte);
            write!(writer, "{:02x}\n", byte).unwrap();
        }
    }

    println!("size is 0x{:x}", current_addr - base_addr.unwrap());
}
