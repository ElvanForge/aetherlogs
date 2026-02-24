use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.evtc")?;

    // 1. Magic (4 bytes)
    file.write_all(b"EVTC")?;

    // 2. Version (8 bytes)
    file.write_all(b"20260224")?;

    // 3. Agent Count (4 bytes, Little Endian)
    // This represents the number 3 (03 00 00 00)
    let agent_count: u32 = 3;
    file.write_all(&agent_count.to_le_bytes())?;

    println!("test.evtc created successfully.");
    Ok(())
}