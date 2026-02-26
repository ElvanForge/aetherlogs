use crate::model::{EvtcHeader, RawAgent};
use std::mem;

pub fn process_log(file_content: &[u8]) {
    //Header
    let header_size = mem::size_of::<EvtcHeader>();
    let (header_data, after_header) = file_content.split_at(header_size);

    let header = unsafe { &*(header_data.as_ptr() as *const EvtcHeader) };
    let boss_id = header.boss_id;
    println!("Boss ID: {}", boss_id);

    //Agent Count
    let (count_data, after_count) = after_header.split_at(4);
    let agent_count = u32::from_le_bytes(count_data.try_into().unwrap());
    println!("Found {} agents in this log.", agent_count);

    //Agent Loop
    let agent_size = mem::size_of::<RawAgent>();

    for (i, agent_chunk) in after_count.chunks_exact(agent_size).take(agent_count as usize).enumerate() {
        let agent = unsafe { &*(agent_chunk.as_ptr() as *const RawAgent) };
        let addr = agent.addr;
        println!("Agent {}: Addr: {}", i, addr);
    }
}