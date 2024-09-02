use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

use crate::encoded_char::{EncodedChar, Encoding};
use crate::segment::Segment;
use crate::utils::smart_encoding_map::SMART_ENCODING_MAP;
use crate::utils::utils::is_gsm7_character;

#[derive(Debug, Clone)]
pub struct SegmentedMessage {
    pub encoding: Encoding,
    pub segments: Vec<Segment>,
    pub graphemes: Vec<String>,
    pub encoding_name: Encoding,
    pub number_of_unicode_scalars: usize,
    pub number_of_characters: usize,
    pub encoded_chars: Vec<EncodedChar>,
    pub line_break_style: Option<LineBreakStyle>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineBreakStyle {
    LF,
    CRLF,
    LFPlusCRLF,
}

impl SegmentedMessage {
    pub fn new(message: &str, encoding: Encoding, smart_encoding: bool) -> Result<Self, String> {
        let message = if smart_encoding {
            message
                .chars()
                .map(|c| *SMART_ENCODING_MAP.get(&c).unwrap_or(&c))
                .collect::<String>()
        } else {
            message.to_string()
        };

        let graphemes = message
            .graphemes(true)
            .flat_map(|g| {
                if g == "\r\n" {
                    vec!["\r".to_string(), "\n".to_string()]
                } else {
                    vec![g.to_string()]
                }
            })
            .collect::<Vec<String>>();

        let number_of_unicode_scalars = message.chars().count();

        let encoding_name = if encoding == Encoding::Auto {
            if Self::has_any_ucs_characters(&graphemes) {
                Encoding::UCS2
            } else {
                Encoding::GSM7
            }
        } else {
            if encoding == Encoding::GSM7 && Self::has_any_ucs_characters(&graphemes) {
                return Err("The string provided is incompatible with GSM-7 encoding".to_string());
            }
            encoding
        };

        let encoded_chars = Self::encode_chars(&graphemes, encoding_name);

        let number_of_characters = if encoding_name == Encoding::UCS2 {
            graphemes.len()
        } else {
            Self::count_code_units(&encoded_chars)
        };

        let segments = Self::build_segments(&encoded_chars);

        let line_break_style = Self::detect_line_break_style(&message);

        let warnings = Self::check_for_warnings(&line_break_style);

        Ok(SegmentedMessage {
            encoding,
            segments,
            graphemes,
            encoding_name,
            number_of_unicode_scalars,
            number_of_characters,
            encoded_chars,
            line_break_style,
            warnings,
        })
    }

    fn has_any_ucs_characters(graphemes: &[String]) -> bool {
        // UTF-8 uses 3 bytes to encode '€', which fails the length check
        // That is the only GSM-7 character encoded with more than 2 byte
        // so we provide a carve out for it
        graphemes
            .iter()
            .any(|g| g.encode_utf16().count() > 1 || !g.chars().all(is_gsm7_character))
    }

    fn encode_chars(graphemes: &[String], encoding: Encoding) -> Vec<EncodedChar> {
        graphemes
            .iter()
            .map(|g| EncodedChar::new(g, encoding))
            .collect()
    }

    fn count_code_units(encoded_chars: &[EncodedChar]) -> usize {
        encoded_chars.iter().map(|ec| ec.code_units.len()).sum()
    }

    fn build_segments(encoded_chars: &[EncodedChar]) -> Vec<Segment> {
        let mut segments = vec![Segment::new(false)];
        let mut current_segment = 0;

        for encoded_char in encoded_chars {
            let char_size = encoded_char.size_in_bits();
            let free_size = segments[current_segment].free_size_in_bits();

            if free_size < char_size as i16 {
                // Start a new segment
                segments.push(Segment::new(true));
                current_segment += 1;

                // Add header to the previous segment if it doesn't have one
                let previous_segment = &mut segments[current_segment - 1];
                if !previous_segment.has_user_data_header {
                    let removed_chars = previous_segment.add_header();
                    segments[current_segment]
                        .extend(removed_chars.into_iter().map(|c| c.clone_box()));
                }
            }

            segments[current_segment].push(Box::new(encoded_char.clone()));
        }

        // Add header to the last segment if it doesn't have one and there's more than one segment
        if segments.len() > 1 && !segments.last().unwrap().has_user_data_header {
            let last_index = segments.len() - 1;
            segments[last_index].add_header();
        }

        segments
    }

    fn detect_line_break_style(message: &str) -> Option<LineBreakStyle> {
        let has_windows_style = message.contains("\r\n");
        let has_unix_style = message.contains('\n');

        match (has_windows_style, has_unix_style) {
            (true, true) => Some(LineBreakStyle::LFPlusCRLF),
            (true, false) => Some(LineBreakStyle::CRLF),
            (false, true) => Some(LineBreakStyle::LF),
            (false, false) => None,
        }
    }

    fn check_for_warnings(line_break_style: &Option<LineBreakStyle>) -> Vec<String> {
        let mut warnings = Vec::new();
        if line_break_style.is_some() {
            warnings.push(
                "The message has line breaks, the web page utility only supports LF style. \
                If you insert a CRLF it will be converted to LF."
                    .to_string(),
            );
        }
        warnings
    }

    pub fn get_encoding_name(&self) -> Encoding {
        self.encoding_name
    }

    pub fn total_size(&self) -> u16 {
        self.segments.iter().map(|s| s.size_in_bits()).sum()
    }

    pub fn message_size(&self) -> u16 {
        self.segments.iter().map(|s| s.message_size_in_bits()).sum()
    }

    pub fn segments_count(&self) -> usize {
        self.segments.len()
    }

    pub fn get_non_gsm_characters(&self) -> HashSet<String> {
        self.encoded_chars
            .iter()
            .filter(|c| !c.is_gsm7)
            .map(|c| c.raw.clone())
            .collect()
    }
}
