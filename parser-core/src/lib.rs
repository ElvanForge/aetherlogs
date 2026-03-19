pub mod model;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser::read_header;

    #[test]
    fn test_read_sample_header() {
        // Path relative to parser-core directory
        let path = "test_data/sample.evtc"; 
        match read_header(path) {
            Ok(header) => {
                println!("\n--- LOG HEADER ---");
                println!("Magic:   {}", header.magic);
                println!("Version: {}", header.version);
                println!("Boss ID: {}", header.boss_id);
                println!("------------------\n");
            },
            Err(e) => panic!("Failed to read header: {}", e),
        }
    }
}