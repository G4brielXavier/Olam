use std::time::{SystemTime, UNIX_EPOCH};

pub mod fiman;
pub mod hey;


pub fn get_hash_8(input: &[u8]) -> String {

    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
    let hash_space_bytes = &tequel::hash::TequelHash::new().tqlhash_raw(&input)[..4];

    let mut hex_buffer = vec![0u8; 8];
    
    for (i, &byte) in hash_space_bytes.iter().enumerate() {
        hex_buffer[i * 2] = HEX_CHARS[(byte >> 4) as usize];
        hex_buffer[i * 2 + 1] = HEX_CHARS[(byte & 0x0f) as usize];
    }

    unsafe { String::from_utf8_unchecked(hex_buffer) }

}



pub fn get_timestamp() -> u128 {
    
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("");

    let millis = since_the_epoch.as_millis();

    millis

}