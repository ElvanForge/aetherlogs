fn main() {
    // 1. Create a 16-byte header
    // "EVTC" (4) + "20230101" (8) + Revision 1 (1) + Boss 123 (2) + Unused (1)
    let mut mock_log = Vec::new();
    mock_log.extend_from_slice(b"EVTC");       // Magic
    mock_log.extend_from_slice(b"20230101");   // Version
    mock_log.push(0x01);                       // Revision
    mock_log.extend_from_slice(&123u16.to_le_bytes()); // Boss ID (123)
    mock_log.push(0x00);                       // Unused

    // 2. Add the Agent Count (Let's say 2 agents)
    mock_log.extend_from_slice(&2u32.to_le_bytes());

    // 3. Add 2 Agents (96 bytes each = 192 bytes of zeros)
    // We'll just fill them with zeros for now, but set one address
    let mut agent1 = [0u8; 96];
    agent1[0..8].copy_from_slice(&1111u64.to_le_bytes()); // Set addr to 1111
    
    let mut agent2 = [0u8; 96];
    agent2[0..8].copy_from_slice(&2222u64.to_le_bytes()); // Set addr to 2222

    mock_log.extend_from_slice(&agent1);
    mock_log.extend_from_slice(&agent2);

    println!("Created mock log of {} bytes", mock_log.len());
    
    // Now pass this fake memory to your parser
    parser::process_log(&mock_log);
}