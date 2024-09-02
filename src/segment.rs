use crate::encoded_char::EncodedCharTrait;
use crate::user_data_header::UserDataHeader;

#[derive(Debug, Clone)]
pub struct Segment {
    pub data: Vec<Box<dyn EncodedCharTrait>>,
    pub has_twilio_reserved_bits: bool,
    pub has_user_data_header: bool,
}

impl Segment {
    pub fn new(with_user_data_header: bool) -> Self {
        let mut segment = Segment {
            data: Vec::new(),
            has_twilio_reserved_bits: with_user_data_header,
            has_user_data_header: with_user_data_header,
        };

        if with_user_data_header {
            for _ in 0..6 {
                segment.data.push(Box::new(UserDataHeader::new()));
            }
        }

        segment
    }

    pub fn size_in_bits(&self) -> u16 {
        self.data.iter().map(|c| c.size_in_bits()).sum()
    }

    pub fn message_size_in_bits(&self) -> u16 {
        self.data
            .iter()
            .filter(|c| !c.is_user_data_header())
            .map(|c| c.size_in_bits())
            .sum()
    }

    pub fn is_user_data_header(&self) -> bool {
        true
    }
    pub fn free_size_in_bits(&self) -> i16 {
        1120 - self.size_in_bits() as i16
    }

    pub fn add_header(&mut self) -> Vec<Box<dyn EncodedCharTrait>> {
        if self.has_user_data_header {
            return Vec::new();
        }

        let mut left_over_char = Vec::new();
        self.has_twilio_reserved_bits = true;
        self.has_user_data_header = true;

        for _ in 0..6 {
            self.data.insert(0, Box::new(UserDataHeader::new()));
        }

        while self.free_size_in_bits() < 0 {
            if let Some(char) = self.data.pop() {
                left_over_char.insert(0, char);
            }
        }

        left_over_char
    }

    pub fn push(&mut self, item: Box<dyn EncodedCharTrait>) {
        self.data.push(item);
    }

    pub fn extend(&mut self, items: impl IntoIterator<Item = Box<dyn EncodedCharTrait>>) {
        self.data.extend(items);
    }
}
