use message_segment_calculator::{Encoding, SegmentedMessage};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- \"Your message here\"");
        return;
    }

    let message = &args[1];
    let encoding = Encoding::Auto;
    let smart_encoding = false;

    match SegmentedMessage::new(message, encoding, smart_encoding) {
        Ok(segmented_message) => {
            println!("{:#?}", segmented_message);
            println!("Message: \"{}\"", message);
            println!("Message successfully segmented!");
            println!("Number of segments: {}", segmented_message.segments_count());
            println!("Encoding used: {:?}", segmented_message.get_encoding_name());
            println!(
                "Number of characters: {}",
                segmented_message.number_of_characters
            );
            println!(
                "Number of unicode scalars: {}",
                segmented_message.number_of_unicode_scalars
            );
            println!("Total size in bits: {}", segmented_message.total_size());
            println!("Message size in bits: {}", segmented_message.message_size());

            let non_gsm_chars = segmented_message.get_non_gsm_characters();
            if !non_gsm_chars.is_empty() {
                println!("Non-GSM characters found: {:?}", non_gsm_chars);
            } else {
                println!("All characters are GSM-compatible.");
            }
        }
        Err(e) => println!("Error segmenting message: {}", e),
    }
}
