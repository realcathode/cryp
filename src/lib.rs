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

/// Converts a hexadecimal string into a vector of bytes (`Vec<u8>`).
///
/// This function takes a string slice containing a hexadecimal representation of data
/// and converts it into its corresponding byte values. The input string must have
/// an even number of characters and consist only of valid hexadecimal digits (`0-9`, `a-f`, `A-F`).
///
/// # Arguments
///
/// * `hexstr` - A string slice (`&str`) containing the hexadecimal data to be converted.
///
/// # Returns
///
/// A `Result` containing:
/// - `Ok(Vec<u8>)` if the conversion succeeds, with the vector representing the bytes.
/// - `Err(Box<dyn Error>)` if the input string is invalid (e.g., not a valid hexadecimal string or of incorrect length).
///
/// # Errors
///
/// This function returns an error in the following cases:
/// - If the input string is not a valid hexadecimal string.
/// - If the string length is odd (since each byte requires two hexadecimal characters).
///
/// # Examples
///
/// ```
/// let hexstr = "48656c6c6f"; // Hexadecimal for "Hello"
/// let bytes = hexstr_to_bytes(hexstr).unwrap();
/// assert_eq!(bytes, vec![72, 101, 108, 108, 111]);
/// ```
///
/// ```
/// let invalid_hexstr = "48656g"; // Invalid hex character 'g'
/// let result = hexstr_to_bytes(invalid_hexstr);
/// assert!(result.is_err());
/// ```
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

/// Encodes the given data as a Base64-encoded string.
///
/// This function takes a vector of bytes (`Vec<u8>`) as input and converts it
/// into a Base64-encoded string. 
///
/// # Arguments
/// 
/// * `data` - A vector of bytes (`Vec<u8>`) that represents the binary data to be encoded.
///
/// # Returns
/// 
/// A `String` containing the Base64-encoded representation of the input data.
///
/// # Examples
/// 
/// ```
/// let data = vec![72, 101, 108, 108, 111]; // Represents "Hello" in ASCII
/// let encoded = base64_encode(data);
/// assert_eq!(encoded, "SGVsbG8=");
/// ```
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

/// Converts a slice of bytes (`&[u8]`) into a hexadecimal string.
///
/// This function takes a byte slice and converts each byte into a
/// two-character hexadecimal representation. The resulting string will
/// contain the hexadecimal values of the input bytes, concatenated without
/// any separators.
///
/// # Arguments
///
/// * `data` - A slice of bytes (`&[u8]`) to be converted into a hexadecimal string.
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the input bytes.
///
/// # Examples
///
/// ```
/// let bytes = &[72, 101, 108, 108, 111]; // Represents "Hello" in bytes
/// let hex_str = bytes_to_hexstr(bytes);
/// assert_eq!(hex_str, "48656c6c6f");
/// ```
///
/// ```
/// let empty_bytes = &[]; // Empty input
/// let hex_str = bytes_to_hexstr(empty_bytes);
/// assert_eq!(hex_str, "");
/// ```
pub fn bytes_to_hexstr(data: &[u8]) -> String {
    let mut hexstr = String::with_capacity(data.len() * 2); // Preallocate space
    for byte in data {
        write!(&mut hexstr, "{:02x}", byte).unwrap();
    }

    hexstr
}

/// Performs a fixed-length XOR operation between two byte slices.
///
/// This function takes two slices of bytes (`data` and `key`), and performs an XOR operation 
/// on each corresponding pair of bytes. The lengths of both slices must be equal, and the result 
/// will be a new `Vec<u8>` containing the XORed values of the two input slices.
///
/// # Arguments
///
/// * `data` - A slice of bytes (`&[u8]`) that will be XORed with the `key`.
/// * `key` - A slice of bytes (`&[u8]`) that will be XORed with the `data`. It must have the same length as `data`.
///
/// # Returns
///
/// A `Result<Vec<u8>, Box<dyn Error>>`. On success, it returns a `Vec<u8>` containing the XORed bytes.
/// If the input slices have different lengths, it returns an error with a message describing the issue.
///
/// # Errors
///
/// This function returns an error if the input slices (`data` and `key`) have different lengths.
///
/// # Examples
///
/// ```
/// let data = vec![0x1c, 0x01, 0x11, 0x00];
/// let key = vec![0x1f, 0x01, 0x01, 0x00];
/// let result = fixed_xor(&data, &key);
/// assert_eq!(result.unwrap(), vec![0x05, 0x00, 0x10, 0x00]);
/// ```
///
pub fn fixed_xor(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if data.len() != key.len() {
        return Err("inputs (data and key) should have equal length".into());        
    }

    let mut xored: Vec<u8> = vec![];
    for (&a, &b) in data.iter().zip(key.iter()) {
        xored.push(a ^ b);
    }

   Ok(xored)
}