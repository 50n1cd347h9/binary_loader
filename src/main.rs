use std::fs::File;
use std::io::Read;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


#[derive(Debug)]
struct ElfHeader {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16, 
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}



#[derive(Debug)]
enum BinaryType {
    BinTypeAuto,
    BinTypeElf,
    BinTypePe
}

#[derive(Debug)]
enum SectionType {
    SecTypeNone,
    SecTypeCode,
    SecTypeData
}

#[derive(Debug)]
enum BinaryArch {
    ArchNone,
    ArchX86
}

#[derive(Debug)]
enum SymbolType {
    SymTypeUkn,
    SymTypeFunc
}

#[derive(Debug)]
struct Symbol {
    r#type: SymbolType,
    name: String,
    addr: u64
}

#[derive(Debug)]
struct Section<'a> {
    binary: &'a Binary<'a>,
    name: String,
    r#type: SectionType,
    vma: u64,
    size: u64,
    bytes: &'a u8
}

#[derive(Debug)]
struct Binary<'a> {
    filename: String,
    r#type: BinaryType,
    type_str: String,
    arch: BinaryArch,
    arch_str: String,
    bits: u32,
    entry: u64, 
    sections: Vec<Section<'a>>,
    symbols: Vec<Symbol>,
}


impl ElfHeader {
    fn parse_elf_header(buf: &[u8]) -> std::io::Result<Self> {
        let mut cur = Cursor::new(buf);
        let mut e_ident = [0_u8; 16];
        for cc in &mut e_ident {
            *cc = cur.read_u8()?;
        }

        Ok(ElfHeader {
            e_ident: e_ident,
            e_type: cur.read_u16::<LittleEndian>()?,
            e_machine: cur.read_u16::<LittleEndian>()?,
            e_version:cur.read_u32::<LittleEndian>()?,
            e_entry: cur.read_u64::<LittleEndian>()?,
            e_phoff: cur.read_u64::<LittleEndian>()?,
            e_shoff: cur.read_u64::<LittleEndian>()?,
            e_flags: cur.read_u32::<LittleEndian>()?,
            e_ehsize: cur.read_u16::<LittleEndian>()?,
            e_phentsize: cur.read_u16::<LittleEndian>()?,
            e_phnum: cur.read_u16::<LittleEndian>()?,
            e_shentsize: cur.read_u16::<LittleEndian>()?,
            e_shnum: cur.read_u16::<LittleEndian>()?,
            e_shstrndx: cur.read_u16::<LittleEndian>()?
        })
    }

    fn print_elf_header(header: &Self) {
        let mut print_buf = String::new();
        let mut magic = String::new();
        let machine = String::new();

        for byte in header.e_ident {
            magic.push_str(&format!("{:02x} ", byte));
        }

        print_buf.push_str(&format!("Magic: {}\n\
                                    Type: {}\n\
                                    Machine: {}\n\
                                    Version: {}\n\
                                    Entry point address: 0x{:x}\n\
                                    Start of program headers: {} (bytes into file)\n\
                                    Start of section headers: {} (bytes into file)\n\
                                    Flags: {} (hogehoge)\n\
                                    Size of this header: {} (bytes)\n\
                                    Size of program headers: {} (bytes)\n\
                                    Number of program headers: {}\n\
                                    Size of section headers: {} (bytes)\n\
                                    Number of section headers: {}\n\
                                    Section header string table index: {}\n",
                                    magic, header.e_type, header.e_machine, header.e_version, header.e_entry, header.e_phoff, header.e_shoff, header.e_flags, header.e_ehsize, header.e_phentsize, header.e_phnum, header.e_shentsize, header.e_shnum, header.e_shstrndx));

        println!("{}", print_buf);
    }
}

impl Symbol {
}

impl Section<'_> {
    fn contains(section: &Self, addr: u64) -> bool {
        return (addr >= section.vma) && (addr - section.vma < section.size);
    }
}

fn main() -> std::io::Result<()> {
    let mut f = File::open("chall")?;
    let mut buf_elf_header = [0_u8; 64];

    let _ = f.read(&mut buf_elf_header)?;

    let elf_header = ElfHeader::parse_elf_header(&buf_elf_header)?;

    ElfHeader::print_elf_header(&elf_header);

    Ok(())
}
