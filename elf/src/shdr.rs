#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::convert::TryInto;
use std::fmt;
use std::mem;
use std::result;

pub const SHN_UNDEF: (u16, &str) = (0x0000, "UNDEF");
pub const SHN_LORESERVE: (u16, &str) = (0xff00, "LORESERVE");
pub const SHN_LOPROC: (u16, &str) = (0xff00, "LOPROC");
pub const SHN_HIPROC: (u16, &str) = (0xff1f, "HIPROC");
pub const SHN_LOOS: (u16, &str) = (0xff20, "LOOS");
pub const SHN_HIOS: (u16, &str) = (0xff3f, "HIOS");
pub const SHN_ABS: (u16, &str) = (0xfff1, "ABS");
pub const SHN_COMMON: (u16, &str) = (0xfff2, "COMMON");
pub const SHN_XINDEX: (u16, &str) = (0xffff, "XINDEX");
pub const SHN_HIRESERVE: (u16, &str) = (0xffff, "HIRESERVE");

pub const SHT_NULL: (u32, &str) = (0x00000000, "NULL");
pub const SHT_PROGBITS: (u32, &str) = (0x00000001, "PROGBITS");
pub const SHT_SYMTAB: (u32, &str) = (0x00000002, "SYMTAB");
pub const SHT_STRTAB: (u32, &str) = (0x00000003, "STRTAB");
pub const SHT_RELA: (u32, &str) = (0x00000004, "RELA");
pub const SHT_HASH: (u32, &str) = (0x00000005, "HASH");
pub const SHT_DYNAMIC: (u32, &str) = (0x00000006, "DYNAMIC");
pub const SHT_NOTE: (u32, &str) = (0x00000007, "NOTE");
pub const SHT_NOBITS: (u32, &str) = (0x00000008, "NOBITS");
pub const SHT_REL: (u32, &str) = (0x00000009, "REL");
pub const SHT_SHLIB: (u32, &str) = (0x0000000a, "SHLIB");
pub const SHT_DYNSYM: (u32, &str) = (0x0000000b, "DYNSYM");
pub const SHT_INIT_ARRAY: (u32, &str) = (0x0000000e, "INIT_ARRAY");
pub const SHT_FINI_ARRAY: (u32, &str) = (0x0000000f, "FINI_ARRAY");
pub const SHT_PREINIT_ARRAY: (u32, &str) = (0x00000010, "PREINIT_ARRAY");
pub const SHT_GROUP: (u32, &str) = (0x00000011, "GROUP");
pub const SHT_SYMTAB_SHNDX: (u32, &str) = (0x00000012, "SYMTAB_SHNDX");
pub const SHT_LOOS: (u32, &str) = (0x60000000, "LOOS");
pub const SHT_HIOS: (u32, &str) = (0x6fffffff, "HIOS");
pub const SHT_LOPROC: (u32, &str) = (0x70000000, "LOPROC");
pub const SHT_HIPROC: (u32, &str) = (0x7fffffff, "HIPROC");
pub const SHT_LOUSER: (u32, &str) = (0x80000000, "LOUSER");
pub const SHT_HIUSER: (u32, &str) = (0xffffffff, "HIUSER");

#[derive(Debug, Default)]
pub struct Elf_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Elf_Shdr {
    pub fn from_bytes(raw: &[u8], little: bool, bit32:bool) -> result::Result<Self, ()> {
        let mut offset = 0;
        let mut ins = Self::default();

        if little {
            ins.sh_name = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());
        } else {

        }

        Ok(ins)
    }
}