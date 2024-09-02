mod encoded_char;
mod segment;
mod segmented_message;
mod user_data_header;
mod utils;

pub use encoded_char::{EncodedChar, EncodedCharTrait, Encoding};
pub use segment::Segment;
pub use segmented_message::{LineBreakStyle, SegmentedMessage};
pub use user_data_header::UserDataHeader;

pub mod unicode_to_gsm {
    pub use crate::utils::unicode_to_gsm::UNICODE_TO_GSM;
}

pub mod smart_encoding_map {
    pub use crate::utils::smart_encoding_map::SMART_ENCODING_MAP;
}
