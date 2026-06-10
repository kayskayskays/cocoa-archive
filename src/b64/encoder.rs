const BASE_64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub(crate) fn encode(byte: u8) -> u8 {
    BASE_64[(byte & 0b11_1111) as usize]
}

pub(crate) fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes
        .chunks(3)
        .flat_map(|chunk| {
            let n0 = chunk[0];
            let n1 = chunk.get(1).copied().unwrap_or(0);
            let n2 = chunk.get(2).copied().unwrap_or(0);

            let n = (n0 as u32) << 16 | (n1 as u32) << 8 | n2 as u32;

            let mut encoded = [0u8; 4];
            encoded[0] = encode((n >> 18) as u8);
            encoded[1] = encode((n >> 12) as u8);
            encoded[2] = encode((n >> 6) as u8);
            encoded[3] = encode(n as u8);

            for idx in 0..(3 - chunk.len()) {
                encoded[encoded.len() - 1 - idx] = b'=';
            }

            encoded
        })
        .collect::<Vec<u8>>()
}
