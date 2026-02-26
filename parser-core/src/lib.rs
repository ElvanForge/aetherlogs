
pub mod model;   
pub mod parser;  


pub extern "C" fn parse_log(ptr: *const u8, len: usize) {
    //implement the Go-FFI call
}