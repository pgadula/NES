use bitflags::bitflags;
use std::{
    fs::File,
    io::{self, Error, Read, Seek},
    path::{Display, Path},
};

const NES_CONSTANT: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

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
    pub flag_6: FLAG6,
    pub flag_7: FLAG7,
    pub prg_size: u8,
    pub chr_size: u8,
}

impl Cartridge {
    pub fn load_rom(path: &Path) -> Result<Cartridge, Error> {
        let mut result = File::open(path)?;
        let mut buf = Vec::new();
        result.read_to_end(&mut buf)?;
        let prg_size = buf[4];
        let chr_size = buf[5];
        let flag6 = buf[6];
        let flag7 = buf[7];
        validate_nes_constant(&buf)?;
        return Ok(Cartridge {
            bytes: buf,
            prg_size,
            chr_size,
            flag_6: FLAG6::from_bits(flag6).unwrap(),
            flag_7: FLAG7::from_bits(flag7).unwrap(),
        });
    }
     pub fn prg_rom_data(&self)->&[u8]{
        let offset = if self.flag_6.contains(FLAG6::Trainer) {512} else {0}; 
        let start = 16 + offset;
        let end = start + 16384 * self.prg_size as usize;
        &self.bytes[start..end]
    }
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
