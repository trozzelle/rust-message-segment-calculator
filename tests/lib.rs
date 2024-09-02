use message_segment_calculator::{Encoding, SegmentedMessage};

mod basic_tests {
    use super::*;

    #[derive(Debug)]
    struct TestData {
        test_description: &'static str,
        body: &'static str,
        encoding: Encoding,
        segments: usize,
        message_size: u16,
        total_size: u16,
        characters: usize,
        unicode_scalars: usize,
    }

    const TEST_DATA: &[TestData] = &[
        TestData {
            test_description: "GSM-7 in one segment",
            body: "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
            encoding: Encoding::GSM7,
            segments: 1,
            message_size: 1120,
            total_size: 1120,
            characters: 160,
            unicode_scalars: 160,
        },
        TestData {
            test_description: "GSM-7 in two segments",
            body: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901",
            encoding: Encoding::GSM7,
            segments: 2,
            message_size: 1127,
            total_size: 1223,
            characters: 161,
            unicode_scalars: 161,
        },
        TestData {
            test_description: "GSM-7 in three segments",
            body: "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567",
            encoding: Encoding::GSM7,
            segments: 3,
            message_size: 2149,
            total_size: 2293,
            characters: 307,
            unicode_scalars: 307,
        },
        TestData {
            test_description: "UCS-2 message in one segment",
            body: "😜23456789012345678901234567890123456789012345678901234567890123456789",
            encoding: Encoding::UCS2,
            segments: 1,
            message_size: 1120,
            total_size: 1120,
            characters: 69,
            unicode_scalars: 69,
        },
        TestData {
            test_description: "UCS-2 message in two segments",
            body: "😜234567890123456789012345678901234567890123456789012345678901234567890",
            encoding: Encoding::UCS2,
            segments: 2,
            message_size: 1136,
            total_size: 1232,
            characters: 70,
            unicode_scalars: 70,
        },
        TestData {
            test_description: "UCS-2 message in three segments",
            body: "😜2345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234",
            encoding: Encoding::UCS2,
            segments: 3,
            message_size: 2160,
            total_size: 2304,
            characters: 134,
            unicode_scalars: 134,
        },
        TestData {
            test_description: "UCS-2 with two bytes extended characters in one segments boundary",
            body: "🇮🇹234567890123456789012345678901234567890123456789012345678901234567",
            encoding: Encoding::UCS2,
            segments: 1,
            message_size: 1120,
            total_size: 1120,
            characters: 67,
            unicode_scalars: 68,
        },
        TestData {
            test_description: "UCS-2 with extended characters in two segments boundary",
            body: "🇮🇹2345678901234567890123456789012345678901234567890123456789012345678",
            encoding: Encoding::UCS2,
            segments: 2,
            message_size: 1136,
            total_size: 1232,
            characters: 68,
            unicode_scalars: 69,
        },
        TestData {
            test_description: "UCS-2 with four bytes extended characters in one segments boundary",
            body: "🏳️‍🌈2345678901234567890123456789012345678901234567890123456789012345",
            encoding: Encoding::UCS2,
            segments: 1,
            message_size: 1120,
            total_size: 1120,
            characters: 65,
            unicode_scalars: 68,
        },
        TestData {
            test_description: "UCS-2 with four bytes extended characters in two segments boundary",
            body: "🏳️‍🌈23456789012345678901234567890123456789012345678901234567890123456",
            encoding: Encoding::UCS2,
            segments: 2,
            message_size: 1136,
            total_size: 1232,
            characters: 66,
            unicode_scalars: 69,
        },
    ];

    #[test]
    fn test_basic_cases() {
        for test in TEST_DATA.iter() {
            println!("{}", test.test_description);
            let segmented_message =
                SegmentedMessage::new(test.body, Encoding::Auto, false).unwrap();
            assert_eq!(
                segmented_message.get_encoding_name(),
                test.encoding,
                "Encoding mismatch. Expected {:?}, got {:?}",
                test.encoding,
                segmented_message.get_encoding_name()
            );
            assert_eq!(
                segmented_message.segments_count(),
                test.segments,
                "Segment count mismatch. Expected {:?}, got {:?}",
                test.segments,
                segmented_message.segments_count()
            );
            assert_eq!(
                segmented_message.message_size(),
                test.message_size,
                "Message size mismatch. Expected {}, got {}",
                test.message_size,
                segmented_message.message_size()
            );
            assert_eq!(
                segmented_message.total_size(),
                test.total_size,
                "Total size mismatch. Expected {}, got {}",
                test.total_size,
                segmented_message.total_size()
            );
            assert_eq!(
                segmented_message.number_of_unicode_scalars, test.unicode_scalars,
                "Unicode scalar count mismatch. Expected {}, got {}",
                test.unicode_scalars, segmented_message.number_of_unicode_scalars
            );
            assert_eq!(
                segmented_message.number_of_characters, test.characters,
                "Character count mismatch. Expected {}, got {}",
                test.characters, segmented_message.number_of_characters
            );
        }
    }
}

mod gsm7_escape_character_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn assert_segmented_message(
        message: &str,
        expected_encoding: Encoding,
        expected_segments: usize,
        expected_message_size: u16,
        expected_total_size: u16,
    ) {
        let segmented_message = SegmentedMessage::new(message, Encoding::Auto, false).unwrap();

        assert_eq!(
            segmented_message.get_encoding_name(),
            expected_encoding,
            "Encoding mismatch. Expected {:?}, got {:?}",
            expected_encoding,
            segmented_message.get_encoding_name()
        );

        assert_eq!(
            segmented_message.segments_count(),
            expected_segments,
            "Segment count mismatch. Expected {}, got {}",
            expected_segments,
            segmented_message.segments_count()
        );

        assert_eq!(
            segmented_message.message_size(),
            expected_message_size,
            "Message size mismatch. Expected {}, got {}",
            expected_message_size,
            segmented_message.message_size()
        );

        assert_eq!(
            segmented_message.total_size(),
            expected_total_size,
            "Total size mismatch. Expected {}, got {}",
            expected_total_size,
            segmented_message.total_size()
        );
    }

    const GSM7_ESCAPE_CHARS: &[char] = &['|', '^', '€', '{', '}', '[', ']', '~', '\\'];

    #[test]
    fn test_one_segment_with_escape_character() {
        for &escape_char in GSM7_ESCAPE_CHARS.iter() {
            let test_message = format!("{}12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678", escape_char);
            assert_segmented_message(&test_message, Encoding::GSM7, 1, 1120, 1120)
        }
    }

    #[test]
    fn test_two_segments_with_escape_character() {
        for &escape_char in GSM7_ESCAPE_CHARS.iter() {
            let test_message = format!("{}123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789", escape_char);
            assert_segmented_message(&test_message, Encoding::GSM7, 2, 1127, 1223);
        }
    }
}

mod one_grapheme_ucs2_characters_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_CHARACTERS: &[char] = &['Á', 'Ú', 'ú', 'ç', 'í', 'Í', 'ó', 'Ó'];

    #[test]
    fn test_one_segment_70_characters() {
        for &character in TEST_CHARACTERS.iter() {
            let test_message = character.to_string().repeat(70);
            let segmented_message =
                SegmentedMessage::new(&test_message, Encoding::Auto, false).unwrap();
            assert_eq!(segmented_message.segments_count(), 1);
            for encoded_char in &segmented_message.encoded_chars {
                assert!(!encoded_char.is_gsm7);
            }
        }
    }

    #[test]
    fn test_two_segments_71_characters() {
        for &character in TEST_CHARACTERS.iter() {
            let test_message = character.to_string().repeat(71);
            let segmented_message =
                SegmentedMessage::new(&test_message, Encoding::Auto, false).unwrap();
            assert_eq!(segmented_message.segments_count(), 2);
            for encoded_char in &segmented_message.encoded_chars {
                assert!(!encoded_char.is_gsm7);
            }
        }
    }
}

mod special_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ucs2_message_with_special_gsm_characters_in_one_segment() {
        let test_message = "😀]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.segments_count(), 1);
    }

    #[test]
    fn test_ucs2_message_with_special_gsm_characters_in_two_segments() {
        let test_message =
            "😀]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.segments_count(), 2);
    }
}

mod line_break_styles_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_message_with_crlf_line_break_style() {
        let test_message = "\rabcde\r\n123";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.number_of_characters, 11);
    }

    #[test]
    fn test_message_with_lf_line_break_style() {
        let test_message = "\nabcde\n\n123\n";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.number_of_characters, 12);
    }

    #[test]
    fn test_triple_accents_characters_unicode() {
        let test_message = "e\u{0301}\u{0301}\u{0301}";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.number_of_characters, 1);
        assert_eq!(segmented_message.number_of_unicode_scalars, 4);
    }

    #[test]
    fn test_triple_accents_characters_one_segment() {
        let test_message = "e\u{0301}\u{0301}\u{0301}aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.segments_count(), 1);
    }

    #[test]
    fn test_triple_accents_characters_two_segments() {
        let test_message = "e\u{0301}\u{0301}\u{0301}́aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(segmented_message.segments_count(), 2);
    }
}

mod methods_tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    #[test]
    fn test_get_non_gsm_characters() {
        let test_message = "más";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();
        assert_eq!(
            segmented_message.get_non_gsm_characters(),
            HashSet::from([String::from("á")])
        );
    }
}

mod segments_analysis_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_gsm7_segments_analysis() {
        let test_message = "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();

        // Check User Data Header
        for segment in segmented_message.segments.iter().take(3) {
            for encoded_char in segment.data.iter().take(6) {
                assert!(encoded_char.is_user_data_header());
            }
        }

        // Check last segment has only 1 character
        let last_segment = segmented_message.segments.last().unwrap();
        assert_eq!(last_segment.data.len(), 7);
        assert_eq!(last_segment.data.last().unwrap().raw(), "7");
    }

    #[test]
    fn test_ucs2_segments_analysis() {
        let test_message = "😜2345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234";
        let segmented_message = SegmentedMessage::new(test_message, Encoding::Auto, false).unwrap();

        // Check User Data Header
        for segment in segmented_message.segments.iter().take(3) {
            for encoded_char in segment.data.iter().take(6) {
                assert!(encoded_char.is_user_data_header());
            }
        }

        // Check last segment has only 1 character
        let last_segment = segmented_message.segments.last().unwrap();
        assert_eq!(last_segment.data.len(), 7);
        assert_eq!(last_segment.data.last().unwrap().raw(), "4");
    }
}
