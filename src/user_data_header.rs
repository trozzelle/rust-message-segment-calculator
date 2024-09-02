use crate::encoded_char::EncodedCharTrait;

#[derive(Debug, Clone)]
pub struct UserDataHeader {
    pub is_reserved_char: bool,
    pub is_user_data_header: bool,
}

impl UserDataHeader {
    pub fn new() -> Self {
        UserDataHeader {
            is_reserved_char: true,
            is_user_data_header: true,
        }
    }

    pub fn code_unit_size_in_bits() -> u8 {
        8
    }

    pub fn is_user_data_header(&self) -> bool {
        true
    }
}

impl EncodedCharTrait for UserDataHeader {
    fn size_in_bits(&self) -> u16 {
        8
    }

    fn clone_box(&self) -> Box<dyn EncodedCharTrait> {
        Box::new(self.clone())
    }

    fn is_user_data_header(&self) -> bool {
        true
    }

    fn raw(&self) -> &str {
        ""
    }
}
