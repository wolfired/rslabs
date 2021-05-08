#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::fs;
use std::process;

use elf;

fn main() {
    for arg in env::args().skip(1) {
        let raw = fs::read(arg.as_str()).unwrap();
        if let Ok(ehdr) = elf::ehdr::Elf_Ehdr::from_bytes(raw.as_slice()) {
            println!("{}", ehdr);

            // let status = process::Command::new(arg).status().unwrap();
        } else {
            println!("Not a ELF file: {}", arg);
        }
    }
}
