pub mod writer;

const BASE_64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub(crate) fn encode(byte: u8) -> u8 {
    BASE_64[byte as usize]
}

pub(crate) fn decode(byte: u8) -> Option<u8> {
    BASE_64.iter().position(|&b| b == byte).map(|i| i as u8)
}