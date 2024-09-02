use crate::utils::unicode_to_gsm::UNICODE_TO_GSM;
use crate::utils::utils::is_gsm7_character;

/// Encoded Character Class
///
/// Utility class to represent a character in a given encoding

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Encoding {
    GSM7,
    UCS2,
    Auto,
}

#[derive(Debug, Clone)]
pub struct EncodedChar {
    pub raw: String,
    pub code_units: Vec<u16>,
    pub is_gsm7: bool,
    pub encoding: Encoding,
}

pub trait EncodedCharTrait: std::fmt::Debug {
    fn size_in_bits(&self) -> u16;
    fn clone_box(&self) -> Box<dyn EncodedCharTrait>;
    fn is_user_data_header(&self) -> bool;
    fn raw(&self) -> &str;
}

impl Clone for Box<dyn EncodedCharTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl EncodedChar {
    pub fn new(char: &str, encoding: Encoding) -> Self {
        let is_gsm7 = char.chars().all(is_gsm7_character);
        let code_units = if is_gsm7 {
            UNICODE_TO_GSM[&(char.chars().next().unwrap() as u32)]
                .iter()
                .map(|&x| x as u16)
                .collect()
        } else {
            char.encode_utf16().collect()
        };

        EncodedChar {
            raw: char.to_string(),
            code_units,
            is_gsm7,
            encoding,
        }
    }
    pub fn code_unit_size_in_bits(&self) -> u16 {
        match self.encoding {
            Encoding::GSM7 => 7,
            Encoding::UCS2 => 8,
            Encoding::Auto => panic!("Encoding::Auto should not be used here"),
        }
    }

    pub fn size_in_bits(&self) -> u16 {
        if self.encoding == Encoding::UCS2 && self.is_gsm7 {
            // GSM characters always use 16 bits in UCS2 encoding
            16
        } else {
            match self.encoding {
                Encoding::GSM7 => self.code_units.len() as u16 * 7,
                Encoding::UCS2 => self.code_units.len() as u16 * 16,
                Encoding::Auto => panic!("Encoding::Auto should not be used here"),
            }
        }
    }
}

impl EncodedCharTrait for EncodedChar {
    fn size_in_bits(&self) -> u16 {
        self.size_in_bits()
    }

    fn clone_box(&self) -> Box<dyn EncodedCharTrait> {
        Box::new(self.clone())
    }

    fn is_user_data_header(&self) -> bool {
        false
    }

    fn raw(&self) -> &str {
        &self.raw
    }
}
