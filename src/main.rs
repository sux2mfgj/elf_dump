extern crate elf;

use std::path::PathBuf;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough argument");
    }
    let path = PathBuf::from(&args[1]);

    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_section =
        match file.get_section(".text")
        {
            Some(s) => s,
            None    => panic!("cannot found .text section"),
        };


    /* let mut current_addr = 0; */

    for _ in 0..text_section.shdr.addr
    {
        println!("00");
        /* println!("{:08x} 00", current_addr); */
        /* current_addr += 1; */
    }

    for d in &text_section.data
    {
        println!("{:02x}", d);
        /* println!("{:08x} {:02x}", current_addr, data); */
        /* current_addr += 1; */
    }

    if file.get_section(".data").is_none() {
        return
    }

    let data_section = match file.get_section(".data")
        {
            Some(s) => s,
            None    => panic!("??"),
        };

    let diff_text_data = data_section.shdr.addr
        - (text_section.shdr.addr + text_section.shdr.size);
    for _ in 0..diff_text_data
    {
        println!("00");
        /* println!("{:08x} 00", current_addr); */
        /* current_addr += 1; */
    }

    for d in &data_section.data
    {
        println!("{:02x}", d);
        /* println!("{:08x} {:02x}", current_addr, d); */
        /* current_addr += 1; */
    }
}
