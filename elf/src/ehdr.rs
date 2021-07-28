#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::result;

use crate::phdr;
use crate::shdr;

/// File identification
pub const EI_MAG0: usize = 0x00;
/// File identification
pub const EI_MAG1: usize = 0x01;
/// File identification
pub const EI_MAG2: usize = 0x02;
/// File identification
pub const EI_MAG3: usize = 0x03;
/// File class
pub const EI_CLASS: usize = 0x04;
/// Data encoding
pub const EI_DATA: usize = 0x05;
/// File version
pub const EI_VERSION: usize = 0x06;
/// Operating system/ABI identification
pub const EI_OSABI: usize = 0x07;
/// ABI version
pub const EI_ABIVERSION: usize = 0x08;
/// Start of padding bytes
pub const EI_PAD: usize = 0x09;
/// Size of e_ident[]
pub const EI_NIDENT: usize = 0x10;

pub const ELFMAG0: (u8, &str) = (0x7f, "0x7f");
pub const ELFMAG1: (u8, &str) = (b'E', "E");
pub const ELFMAG2: (u8, &str) = (b'L', "L");
pub const ELFMAG3: (u8, &str) = (b'F', "F");

pub const ELFCLASSNONE: (u8, &str) = (0x00, "Invalid class");
pub const ELFCLASS32: (u8, &str) = (0x01, "32-bit objects");
pub const ELFCLASS64: (u8, &str) = (0x02, "64-bit objects");

pub const ELFDATANONE: (u8, &str) = (0x00, "Invalid data encoding");
pub const ELFDATA2LSB: (u8, &str) = (0x01, "little endian");
pub const ELFDATA2MSB: (u8, &str) = (0x02, "big endian");

pub const ELFOSABI_NONE: (u8, &str) = (0x00, "No extensions or unspecified");
pub const ELFOSABI_HPUX: (u8, &str) = (0x01, "Hewlett-Packard HP-UX");
pub const ELFOSABI_NETBSD: (u8, &str) = (0x02, "NetBSD");
pub const ELFOSABI_LINUX: (u8, &str) = (0x03, "Linux");
pub const ELFOSABI_SOLARIS: (u8, &str) = (0x06, "Sun Solaris");
pub const ELFOSABI_AIX: (u8, &str) = (0x07, "AIX");
pub const ELFOSABI_IRIX: (u8, &str) = (0x08, "IRIX");
pub const ELFOSABI_FREEBSD: (u8, &str) = (0x09, "FreeBSD");
pub const ELFOSABI_TRU64: (u8, &str) = (0x0a, "Compaq TRU64 UNIX");
pub const ELFOSABI_MODESTO: (u8, &str) = (0x0b, "Novell Modesto");
pub const ELFOSABI_OPENBSD: (u8, &str) = (0x0c, "Open BSD");
pub const ELFOSABI_OPENVMS: (u8, &str) = (0x0d, "Open VMS");
pub const ELFOSABI_NSK: (u8, &str) = (0x0e, "Hewlett-Packard Non-Stop Kernel");

pub const ET_NONE: (u16, &str) = (0x0000, "No file type");
pub const ET_REL: (u16, &str) = (0x0001, "Relocatable file");
pub const ET_EXEC: (u16, &str) = (0x0002, "Executable file");
pub const ET_DYN: (u16, &str) = (0x0003, "Shared object file");
pub const ET_CORE: (u16, &str) = (0x0004, "Core file");
pub const ET_LOOS: (u16, &str) = (0xfe00, "Operating system-specific");
pub const ET_HIOS: (u16, &str) = (0xfeff, "Operating system-specific");
pub const ET_LOPROC: (u16, &str) = (0xff00, "Processor-specific");
pub const ET_HIPROC: (u16, &str) = (0xffff, "Processor-specific");

pub const EM_NONE: (u16, &str) = (0x0000, "No machine");
pub const EM_386: (u16, &str) = (0x0003, "Intel 80386");
pub const EM_X86_64: (u16, &str) = (0x003e, "AMD x86-64 architecture");

pub const EV_NONE: (u32, &str) = (0x00000000, "Invalid version");
pub const EV_CURRENT: (u32, &str) = (0x00000001, "Current version");

#[derive(Debug, Default)]
pub struct Elf_Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
    pub phdrs: Vec<phdr::Elf_Phdr>,
    pub shdrs: Vec<shdr::Elf_Shdr>,
}

impl Elf_Ehdr {
    pub fn from_bytes(raw: &[u8]) -> result::Result<Self, ()> {
        if EI_NIDENT > raw.len() {
            return Err(());
        }

        let mut offset = 0;
        let mut ins = Self::default();
        ins.e_ident = (&raw[offset..{offset += EI_NIDENT; offset}]).try_into().unwrap();

        if ELFMAG0.0 != ins.e_ident[EI_MAG0] || ELFMAG1.0 != ins.e_ident[EI_MAG1] || ELFMAG2.0 != ins.e_ident[EI_MAG2] || ELFMAG3.0 != ins.e_ident[EI_MAG3] {
            return Err(());
        }
    
        if ELFDATA2LSB.0 == ins.e_ident[EI_DATA] {
            ins.e_type = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_machine = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_version = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());
            if ELFCLASS32.0 == ins.e_ident[EI_CLASS] {
                ins.e_entry = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                ins.e_phoff = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                ins.e_shoff = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
            } else {
                ins.e_entry = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                ins.e_phoff = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                ins.e_shoff = u64::from_le_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
            }
            ins.e_flags = u32::from_le_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());
            ins.e_ehsize = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_phentsize = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_phnum = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_shentsize = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_shnum = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_shstrndx = u16::from_le_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
        } else if ELFDATA2MSB.0 == ins.e_ident[EI_DATA] {
            ins.e_type = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_machine = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_version = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());
            if ELFCLASS32.0 == ins.e_ident[EI_CLASS] {
                ins.e_entry = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                ins.e_phoff = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
                ins.e_shoff = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap()) as u64;
            } else {
                ins.e_entry = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                ins.e_phoff = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
                ins.e_shoff = u64::from_be_bytes((&raw[offset..{offset += 8; offset}]).try_into().unwrap());
            }
            ins.e_flags = u32::from_be_bytes((&raw[offset..{offset += 4; offset}]).try_into().unwrap());
            ins.e_ehsize = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_phentsize = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_phnum = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_shentsize = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_shnum = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
            ins.e_shstrndx = u16::from_be_bytes((&raw[offset..{offset += 2; offset}]).try_into().unwrap());
        } else {
            return Err(());
        }

        offset = ins.e_phoff as usize;
        for _ in 0..ins.e_phnum {
            let r = phdr::Elf_Phdr::from_bytes(&raw[offset..{offset += ins.e_phentsize as usize; offset}], ELFDATA2LSB.0 == ins.e_ident[EI_DATA], ELFCLASS32.0 == ins.e_ident[EI_CLASS]);
            if let Ok(phdr) = r {
                ins.phdrs.push(phdr);
            }
        }

        offset = ins.e_shoff as usize;
        for _ in 0..ins.e_shnum {
            let r = shdr::Elf_Shdr::from_bytes(&raw[offset..{offset += ins.e_shentsize as usize; offset}], ELFDATA2LSB.0 == ins.e_ident[EI_DATA], ELFCLASS32.0 == ins.e_ident[EI_CLASS]);
            if let Ok(shdr) = r {
                ins.shdrs.push(shdr);
            }
        }

        Ok(ins)
    }
}

impl fmt::Display for Elf_Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:>36}: {}", "Magic", "ELF")?;
        writeln!(f, "{:>36}: {}", "Class", match self.e_ident[EI_CLASS] {
            class if ELFCLASS32.0 == class => ELFCLASS32.1,
            class if ELFCLASS64.0 == class => ELFCLASS64.1,
            _ => ELFCLASSNONE.1,
        })?;
        writeln!(f, "{:>36}: {}", "Data", match self.e_ident[EI_DATA] {
            data if ELFDATA2LSB.0 == data => ELFDATA2LSB.1,
            data if ELFDATA2MSB.0 == data => ELFDATA2MSB.1,
            _ => ELFDATANONE.1,
        })?;
        writeln!(f, "{:>36}: {:#04x} (current)", "Version", self.e_ident[EI_VERSION])?;
        writeln!(f, "{:>36}: {}", "OS/ABI", match self.e_ident[EI_OSABI] {
            osabi if ELFOSABI_HPUX.0 == osabi => ELFOSABI_HPUX.1,
            _ => ELFOSABI_NONE.1,
        })?;
        writeln!(f, "{:>36}: {:#04x}", "ABI Version", self.e_ident[EI_ABIVERSION])?;
        writeln!(f, "{:>36}: {}", "Type", match self.e_type {
            r#type if ET_REL.0 == r#type => ET_REL.1,
            r#type if ET_EXEC.0 == r#type => ET_EXEC.1,
            r#type if ET_DYN.0 == r#type => ET_DYN.1,
            r#type if ET_CORE.0 == r#type => ET_CORE.1,
            r#type if ET_LOOS.0 == r#type => ET_LOOS.1,
            r#type if ET_HIOS.0 == r#type => ET_HIOS.1,
            r#type if ET_LOPROC.0 == r#type => ET_LOPROC.1,
            r#type if ET_HIPROC.0 == r#type => ET_HIPROC.1,
            _ => ET_NONE.1,
        })?;
        writeln!(f, "{:>36}: {}", "Machine", match self.e_machine {
            machine if EM_386.0 == machine => EM_386.1,
            machine if EM_X86_64.0 == machine => EM_X86_64.1,
            _ => EM_NONE.1,
        })?;
        writeln!(f, "{:>36}: {}", "Version", match self.e_version {
            version if EV_CURRENT.0 == version => EV_CURRENT.1,
            _ => EV_NONE.1,
        })?;
        writeln!(f, "{:>36}: {:#02$x}", "Entry point address", self.e_entry, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
        writeln!(f, "{:>36}: {} {}", "Start of program headers", self.e_phoff, "(bytes into file)")?;
        writeln!(f, "{:>36}: {} {}", "Start of section headers", self.e_shoff, "(bytes into file)")?;
        writeln!(f, "{:>36}: {:#010x}", "Flags", self.e_flags)?;
        writeln!(f, "{:>36}: {} {}", "Size of this header", self.e_ehsize, "(bytes)")?;
        writeln!(f, "{:>36}: {} {}", "Size of program headers", self.e_phentsize, "(bytes)")?;
        writeln!(f, "{:>36}: {}", "Number of program headers", self.e_phnum)?;
        writeln!(f, "{:>36}: {} {}", "Size of section headers", self.e_shentsize, "(bytes)")?;
        writeln!(f, "{:>36}: {}", "Number of section headers", self.e_shnum)?;
        writeln!(f, "{:>36}: {}", "Section header string table index", self.e_shstrndx)?;

        writeln!(f)?;
        writeln!(f, "{:^10} {:^8$} {:^8$} {:^8$} {:^8$} {:^8$} {:^10} {:^8$}", "Type", "Offset", "VirtAddr", "PhysAddr", "FileSiz", "MemSiz", "Flg", "Align", 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;

        for phdr in self.phdrs.iter() {
            write!(f, "{:^10} ", match phdr.p_type {
                version if phdr::PT_LOAD.0 == version => phdr::PT_LOAD.1,
                version if phdr::PT_DYNAMIC.0 == version => phdr::PT_DYNAMIC.1,
                version if phdr::PT_INTERP.0 == version => phdr::PT_INTERP.1,
                version if phdr::PT_NOTE.0 == version => phdr::PT_NOTE.1,
                version if phdr::PT_SHLIB.0 == version => phdr::PT_SHLIB.1,
                version if phdr::PT_PHDR.0 == version => phdr::PT_PHDR.1,
                version if phdr::PT_TLS.0 == version => phdr::PT_TLS.1,
                version if phdr::PT_LOOS.0 == version => phdr::PT_LOOS.1,
                version if phdr::PT_HIOS.0 == version => phdr::PT_HIOS.1,
                version if phdr::PT_LOPROC.0 == version => phdr::PT_LOPROC.1,
                version if phdr::PT_HIPROC.0 == version => phdr::PT_HIPROC.1,
                _ => phdr::PT_NULL.1,
            })?;
            write!(f, "{:#01$x} ", phdr.p_offset, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
            write!(f, "{:#01$x} ", phdr.p_vaddr, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
            write!(f, "{:#01$x} ", phdr.p_paddr, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
            write!(f, "{:#01$x} ", phdr.p_filesz, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
            write!(f, "{:#01$x} ", phdr.p_memsz, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
            write!(f, "{:^#10b} ", phdr.p_flags)?;
            writeln!(f, "{:#01$x} ", phdr.p_align, 2 + if ELFCLASS32.0 == self.e_ident[EI_CLASS] { 4 } else { 8 } * 2)?;
        }

        writeln!(f)?;
        writeln!(f, "{:^10} {:^8} {:^8} {:^8} {:^8} {:^8} {:^8} {:^8} {:^8} {:^8} {:^8}", "[Nr]", "Name", "Type", "Addr", "Off", "Size", "Es", "Flg", "Lk", "Inf", "Al")?;

        for (index, shdr) in self.shdrs.iter().enumerate() {
            write!(f, "{:^10} ", index)?;
            writeln!(f, "{:^10} ", shdr.sh_name)?;
        }

        writeln!(f)
    }
}
