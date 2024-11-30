use std::error::Error;
use std::fmt::Write;
use std::collections::HashMap;
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
/// let result = xor_fixed(&data, &key);
/// assert_eq!(result.unwrap(), vec![0x05, 0x00, 0x10, 0x00]);
/// ```
pub fn xor_fixed(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if data.len() != key.len() {
        return Err("inputs (data and key) should have equal length".into());        
    }

    let mut xored: Vec<u8> = vec![];
    for (&a, &b) in data.iter().zip(key.iter()) {
        xored.push(a ^ b);
    }

   Ok(xored)
}

/// Applies a one-byte XOR operation to a slice of bytes.
///
/// This function takes a slice of bytes and a single byte key and XORs 
/// each byte in the slice with the key. The result is returned as a 
/// new vector of bytes.
///
/// # Arguments
///
/// * `data` - A slice of bytes (`&[u8]`) that will be XORed.
/// * `key` - A single byte (`u8`) used as the XOR key.
///
/// # Returns
///
/// A `Vec<u8>` containing the XORed bytes.
///
/// # Examples
///
/// ```
/// let data = b"Hello";
/// let key = 42; // Example key
/// let result = xor_one_byte(data, key);
/// ```
pub fn xor_one_byte(data: &[u8], key: u8) -> Vec<u8> {
    let mut xored: Vec<u8> = vec![];
    for &a in data.iter() {
        xored.push(a ^ key);
    }
    
    xored
}

/// Performs a repeating XOR operation on the given data using the provided key.
/// 
/// The key is applied cyclically to the data. If the key is shorter than the data,
/// it is repeated in a loop to match the length of the data. Each byte of the data is
/// XORed with the corresponding byte of the key, using the modulus of the key length
/// to repeat the key as necessary.
///
/// # Arguments
/// 
/// * `data` - A byte slice (`&[u8]`) representing the data to be XORed.
/// * `key` - A byte slice (`&[u8]`) representing the key used for the XOR operation.
/// 
/// # Returns
/// 
/// A `Vec<u8>` containing the result of the XOR operation, where each byte in the result
/// is the XOR of the corresponding byte in the data and the key. The result will have the same
/// length as the input data.
///
pub fn xor_repeating(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut xored: Vec<u8> = vec![];
    for (i, &byte) in data.iter().enumerate() {
        xored.push(byte ^ key[i % key.len()]);
    }

    xored
}

/// Scores the text based on the proportion of alphanumeric and space characters.
///
/// This function calculates a score for the input data by determining the percentage
/// of characters that are ASCII alphanumeric or spaces. The score is returned as a
/// floating-point value.
///
/// # Arguments
///
/// * `data` - A byte slice (`&[u8]`) representing the input data.
///
/// # Returns
///
/// A `f32` representing the percentage of valid ASCII alphanumeric and space characters.
///
/// # Examples
///
/// ```
/// let data = b"Hello, world!";
/// let score = score_text(data);
/// assert!(score > 50.0);
/// ```
pub fn score_text(data: &[u8]) -> f32 {
    let valid = data
        .iter()
        .filter(|&&byte| (byte as char).is_ascii_alphanumeric() || byte == b' ')
        .count();

    (100.0 * valid as f32) / data.len() as f32
}

/// Analyzes a given string and calculates the ratios of letters, spaces, and symbols.
///
/// # Parameters
/// - `s`: A string slice (`&str`) representing the input text to analyze.
///
/// # Returns
/// A tuple `(f64, f64, f64)` containing:
/// - `letter_ratio`: The proportion of alphabetic characters in the string.
/// - `space_ratio`: The proportion of whitespace characters in the string.
/// - `symbol_ratio`: The proportion of non-alphabetic and non-whitespace characters.
///
fn analyze_string(s: &str) -> (f64, f64, f64) {
    let total_chars = s.len() as f64;
    let letters = s.chars().filter(|c| c.is_ascii_alphabetic()).count() as f64;
    let spaces = s.chars().filter(|c| c.is_whitespace()).count() as f64;
    let symbols = total_chars - letters - spaces;

    let letter_ratio = letters / total_chars;
    let space_ratio = spaces / total_chars;
    let symbol_ratio = symbols / total_chars;

    (letter_ratio, space_ratio, symbol_ratio)
}

/// Validates whether a given byte slice (`&[u8]`) represents valid English-like text.
///
/// # Parameters
/// - `s`: A byte slice (`&[u8]`) containing the input data to validate.
///
/// # Returns
/// - `true` if the input satisfies the following conditions:
///   - Letters make up at least 70% of the total characters.
///   - Spaces make up at most 20% of the total characters.
///   - Symbols make up at most 10% of the total characters.
/// - `false` otherwise.
///
/// # Behavior
/// - Invalid UTF-8 sequences are replaced with the `�` character (`U+FFFD`) by using
///   `String::from_utf8_lossy`. 
/// 
/// # TODO
/// - Exclude the replacement character (`�`) from the analysis if this behavior
///   is not desired in the future.
///
pub fn is_valid_text(s: &[u8]) -> bool {
    let s = String::from_utf8_lossy(&s);
    let (letter_ratio, space_ratio, symbol_ratio) = analyze_string(&s);

    letter_ratio >= 0.7 &&
    space_ratio <= 0.2 &&
    symbol_ratio <= 0.1
}

/// Computes the frequency of ASCII alphabetic characters in a byte slice.
///
/// # Arguments
///
/// * `data` - A slice of bytes (`&[u8]`) to analyze for character frequencies.
///
/// # Returns
///
/// A `HashMap` where the keys are ASCII alphabetic characters (`char`) 
/// and the values are their respective frequencies (`usize`) in the input slice.
///
/// # Behavior
///
/// - Only ASCII alphabetic characters (`A-Z` and `a-z`) are counted.
/// - Characters are converted to lowercase before counting to ensure case insensitivity.
/// - Non-alphabetic characters (e.g., digits, punctuation) are ignored.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
///
/// let data = b"Hello, World!";
/// let result = character_frequency(data);
///
/// let mut expected = HashMap::new();
/// expected.insert('h', 1);
/// expected.insert('e', 1);
/// expected.insert('l', 3);
/// expected.insert('o', 2);
/// expected.insert('w', 1);
/// expected.insert('r', 1);
/// expected.insert('d', 1);
///
/// assert_eq!(result, expected);
/// ```
pub fn character_frequency(data: &[u8]) -> HashMap<char, usize> {
    let mut freq = HashMap::new();

    for &byte in data {
        if byte.to_ascii_lowercase().is_ascii_alphabetic() {
            let ch = byte.to_ascii_lowercase() as char;
            *freq.entry(ch).or_insert(0) += 1;
        }
    }
    freq
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn hamming_distance_bit(s1: &str, s2: &str) -> u32 {
    s1.bytes().zip(s2.bytes())
        .map(|(b1, b2)| (b1 ^ b2).count_ones()).sum()
}

pub fn hamming_distance_char(s1: &str, s2: &str) -> u32 {
    s1.chars().zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2).count() as u32
}