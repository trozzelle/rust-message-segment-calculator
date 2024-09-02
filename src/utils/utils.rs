use crate::utils::unicode_to_gsm::UNICODE_TO_GSM;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref GSM7_MULTIBYTE_EXCEPTIONS: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('€');
        set
    };
}

pub fn is_gsm7_character(c: char) -> bool {
    if GSM7_MULTIBYTE_EXCEPTIONS.contains(&c) {
        true
    } else {
        c.len_utf8() == 1 && UNICODE_TO_GSM.contains_key(&(c as u32))
    }
}
