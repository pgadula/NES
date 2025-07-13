use bitflags::bitflags;
use std::{
    fs::File,
    io::{self, Error, Read},
    path::Path,
};

const NES_CONSTANT: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const MAPPER_MASK: u8 = 0b11110000;

bitflags! {
    #[derive(Debug)]
    pub struct FLAG6: u8 {
       const Nametable =            0b00000001;
       const Battery =             0b00000010;
       const Trainer  = 0b00000100;
       const Alternative_Nametable  = 0b00001000;
       const MAPPER_NUMBER = 0b11110000;
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct FLAG7: u8 {
       const VS_UniSystem =            0b00000001;
       const PlayerChoice10 =             0b00000010;
       const NES_2_0FORMAT = 0b00001100;
       const MAPPER_NUMBER = 0b11110000;
    }
}

#[derive(Debug)]
pub struct Cartridge {
    pub bytes: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub flag_6: FLAG6,
    pub flag_7: FLAG7,
    pub prg_start_addr: usize,
    pub prg_size: u8,
    pub chr_size: u8,
    pub mapper: u8,
}

impl Cartridge {
    pub fn load_rom(path: &Path) -> Result<Cartridge, Error> {
        let mut result = File::open(path)?;
        let mut buf = Vec::new();
        result.read_to_end(&mut buf)?;
        Cartridge::parse_cartridge(buf)
    }

    pub fn load_rom_from_bytes(bytes: &[u8]) -> Result<Cartridge, Error> {
        Cartridge::parse_cartridge(bytes.to_vec())
    }

    pub fn prg_rom_data(&self) -> &[u8] {
        let end = self.prg_start_addr + 16384 * self.prg_size as usize;
        &self.bytes[self.prg_start_addr..end]
    }

    pub fn chr_rom_data(&self) -> &[u8] {
        let offset = if self.flag_6.contains(FLAG6::Trainer) {
            512
        } else {
            0
        };
        let pgr_offset = 16 + offset;
        let start = pgr_offset + 16384 * self.prg_size as usize;
        let end = start + 8192 * self.chr_size as usize;
        &self.bytes[start..end]
    }

    pub fn backgrounds(&self) ->&[u8]{
        return &self.chr_rom_data()[0..4096]
    }


    pub fn sprites(&self) ->&[u8]{
        return &self.chr_rom_data()[4096..]
    }

    pub fn has_trainer(&self) -> bool {
        self.flag_6.contains(FLAG6::Trainer)
    }

    pub fn write(&mut self, addr: usize, value: u8) -> Result<(), Error> {
        if addr >= 0x6000 && addr <= 0x7FFF {
            self.prg_ram[addr.wrapping_sub(0x6000)] = value;
            return Ok(());
        }
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Outside of addressable range.",
        ));
    }

    pub fn read(&self, addr: usize) -> Result<u8, Error> {
        if addr >= 0x6000 && addr <= 0x7FFF {
            return Ok(self.prg_ram[addr.wrapping_sub(0x6000)]);
        }

        if addr >= 0x8000 && addr <= 0xFFFF {
            let prg = self.prg_rom_data();
            let prg_len = prg.len(); // 16KB or 32KB
            let offset = (addr - 0x8000) % prg_len;

            return Ok(prg[offset]);
        }
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Outside of addressable range.",
        ));
    }

    pub fn info(&self) {
        match self.prg_size {
            1 => println!("Detected: NROM-128 (16 KB PRG-ROM)"),
            2 => println!("Detected: NROM-256 (32 KB PRG-ROM)"),
            _ => println!("Non-standard ROM size: x 16KB"),
        }
    }

    fn parse_cartridge(buf: Vec<u8>) -> Result<Cartridge, Error> {
        if buf.len() < 7 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Buffer to short",
            ));
        }
        let prg_size = buf[4];
        let chr_size = buf[5];
        let flag6 = buf[6];
        let flag7 = buf[7];
        let mapper = (flag7 & MAPPER_MASK) | ((flag6 & MAPPER_MASK) >> 4);
        println!("Mapper number: {}", mapper);
        let offset = if FLAG6::from_bits(flag6).unwrap().contains(FLAG6::Trainer) {
            512
        } else {
            0
        };
        let pgr_start_addr = 16 + offset;
        Cartridge::validate_nes_constant(&buf)?;
        return Ok(Cartridge {
            bytes: buf,
            prg_ram: vec![0u8; 0x2000],
            prg_size,
            prg_start_addr: pgr_start_addr,
            chr_size,
            flag_6: FLAG6::from_bits(flag6).unwrap(),
            flag_7: FLAG7::from_bits(flag7).unwrap(),
            mapper,
        });
    }

    fn validate_nes_constant(buf: &Vec<u8>) -> Result<(), io::Error> {
        if buf.len() < NES_CONSTANT.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Buffer too short to contain NES header",
            ));
        }

        for (i, byte) in NES_CONSTANT.iter().enumerate() {
            if buf[i] != *byte {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid NES header at byte {}: expected 0x{:02X}, found 0x{:02X}",
                        i, byte, buf[i]
                    ),
                ));
            }
        }

        Ok(())
    }
}
