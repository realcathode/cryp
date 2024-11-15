use std::error::Error;
use std::fmt::Write;
const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PADDING: char = '=';

fn validate_hexstr(hexstr: &str) -> Result<(), Box<dyn Error>> {
    if hexstr.len() % 2 != 0 {
        return Err("hex not valid: odd length".into());
    }
    Ok(())
}

fn collect_six_bits(from: (u8, u8), offset: u8) -> u8 {
    let combined: u16 = ((from.0 as u16) << 8) | (from.1 as u16);
    ((combined & (0b1111110000000000u16 >> offset)) >> (10 - offset)) as u8
}

pub fn hexstr_to_bytes(hexstr: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if let Err(e) = validate_hexstr(&hexstr) {
        return Err(e);
    }

    let mut bytes: Vec<u8> = Vec::new();    

    for i in (0..hexstr.len()).step_by(2) {
        let byte = u8::from_str_radix(&hexstr[i..][..2], 16).unwrap();
        bytes.push(byte);
    }

    Ok(bytes)
}


pub fn base64_encode(data: &[u8]) -> String {
    let mut bits_encoded = 0usize;
    let mut encoded_string = String::new();
    let padding_needed = ((6 - (data.len() * 8) % 6) / 2) % 3;
    loop {
        let lower_byte_index_to_encode = bits_encoded / 8usize;
        if lower_byte_index_to_encode == data.len() {
            break;
        }
        let lower_byte_to_encode = data[lower_byte_index_to_encode];
        let upper_byte_to_encode = if (lower_byte_index_to_encode + 1) == data.len() {
            0u8
        } else {
            data[lower_byte_index_to_encode + 1]
        };
        let bytes_to_encode = (lower_byte_to_encode, upper_byte_to_encode);
        let offset: u8 = (bits_encoded % 8) as u8;
        encoded_string.push(CHARSET[collect_six_bits(bytes_to_encode, offset) as usize] as char);
        bits_encoded += 6;
    }
    for _ in 0..padding_needed {
        encoded_string.push(PADDING);
    }
    encoded_string
}

pub fn bytes_to_hexstr(data: &[u8]) -> String {
    let mut hexstr = String::with_capacity(data.len() * 2); // Preallocate space
    for byte in data {
        write!(&mut hexstr, "{:02x}", byte).unwrap();
    }

    hexstr
}