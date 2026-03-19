use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::model::EvtcHeader; // Assuming EvtcHeader is in model.rs

pub fn read_header(file_path: &str) -> io::Result<EvtcHeader> {
    let mut file = File::open(file_path)?;

    // 1. Read Magic (4 bytes: "EVTC")
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    let magic_str = String::from_utf8_lossy(&magic).to_string();

    if magic_str != "EVTC" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid EVTC file"));
    }

    // 2. Read Version/Date (8 bytes: "YYYYMMDD")
    let mut version = [0u8; 8];
    file.read_exact(&mut version)?;
    let version_str = String::from_utf8_lossy(&version).to_string();

    // 3. Skip 1 byte (revision) and read Boss ID (u16)
    file.seek(SeekFrom::Current(1))?;
    let boss_id = file.read_u16::<LittleEndian>()?;

    Ok(EvtcHeader {
        magic: magic_str,
        version: version_str,
        boss_id,
    })
}