#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt;
use std::convert::TryInto;
use std::result;
use std::mem;

pub const PT_NULL:(u32, &str) = (0x00000000, "NULL");
pub const PT_LOAD:(u32, &str) = (0x00000001, "LOAD");
pub const PT_DYNAMIC:(u32, &str) = (0x00000002, "DYNAMIC");
pub const PT_INTERP:(u32, &str) = (0x00000003, "INTERP");
pub const PT_NOTE:(u32, &str) = (0x00000004, "NOTE");
pub const PT_SHLIB:(u32, &str) = (0x00000005, "SHLIB");
pub const PT_PHDR:(u32, &str) = (0x00000006, "PHDR");
pub const PT_TLS:(u32, &str) = (0x00000007, "TLS");
pub const PT_LOOS:(u32, &str) = (0x60000000, "LOOS");
pub const PT_HIOS:(u32, &str) = (0x6fffffff, "HIOS");
pub const PT_LOPROC:(u32, &str) = (0x70000000, "LOPROC");
pub const PT_HIPROC:(u32, &str) = (0x7fffffff, "HIPROC");

#[derive(Debug, Default)]
pub struct Elf_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Elf_Phdr {
    pub fn from_bytes(raw: &[u8], little: bool, bit32:bool) -> result::Result<Self, ()> {
        let mut offset = 0;
        let mut phdr = Elf_Phdr::default();

        if little {
            phdr.p_type = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());

            if bit32 {
                phdr.p_offset = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_vaddr = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_paddr = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_filesz = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_memsz = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;

                phdr.p_flags = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());

                phdr.p_align = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
            } else {
                phdr.p_flags = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());

                phdr.p_offset = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_vaddr = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_paddr = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_filesz = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_memsz = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());

                phdr.p_align = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
            }
        } else {
            phdr.p_type = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());

            if bit32 {
                phdr.p_offset = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_vaddr = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_paddr = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_filesz = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                phdr.p_memsz = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;

                phdr.p_flags = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());

                phdr.p_align = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
            } else {
                phdr.p_flags = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());

                phdr.p_offset = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_vaddr = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_paddr = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_filesz = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                phdr.p_memsz = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());

                phdr.p_align = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
            }
        }

        Ok(phdr)
    }
}
