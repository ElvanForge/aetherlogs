use std::fs;
use nom::{
    bytes::complete::take,
    number::complete::le_u32,
    IResult,
};

#[derive(Debug)]
struct EvtcLog {
    magic: String,
    version: String,
    agent_count: u32,
}

fn parse_evtc(input: &[u8]) -> IResult<&[u8], EvtcLog> {
    let (input, magic_bytes) = take(4usize)(input)?;
    let (input, version_bytes) = take(8usize)(input)?;
    let (input, agent_count) = le_u32(input)?;

    Ok((input, EvtcLog {
        magic: String::from_utf8_lossy(magic_bytes).into_owned(),
        version: String::from_utf8_lossy(version_bytes).into_owned(),
        agent_count,
    }))
}

fn main() {
    
    let file_path = "test.evtc"; 

    match fs::read(file_path) {
        Ok(bytes) => {
            println!("Read {} bytes from file.", bytes.len());

      
            match parse_evtc(&bytes) {
                Ok((_remaining, log)) => {
                    println!("--- Parsed Successfully ---");
                    println!("{:#?}", log);
                }
                Err(e) => println!("Failed to parse EVTC structure: {:?}", e),
            }
        }
        Err(e) => {
            println!("Could not open file '{}': {}", file_path, e);
        }
    }
}