A Rust implementation of
Twilio's [Message Segment Calculator](https://github.com/TwilioDevEd/message-segment-calculator/tree/main), originally
written in JavaScript.

## Overview

This library provides functionality to segment SMS messages, detect their character set, encoding, and other properties.
It's particularly useful for applications that need to work with SMS messages at a low level, ensuring proper encoding
and segmentation for delivery.

## Features

- Segment SMS messages
- Detect character encoding (GSM-7 or UCS-2)
- Smart character encoding
- Handle User Data Headers
- Calculate message size and segment count

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
message_segment_calculator = "0.1.0"
```

## Usage

Here's a basic example of how to use the library:

```rust
use message_segment_calculator::{SegmentedMessage, Encoding};

fn main() {
    let message = "Hello, world! 👋";
    let encoding = Encoding::Auto;
    let smart_encoding = true;

    match SegmentedMessage::new(message, encoding, smart_encoding) {
        Ok(segmented_message) => {
            println!("Encoding: {:?}", segmented_message.get_encoding_name());
            println!("Number of segments: {}", segmented_message.segments_count());
            println!("Total size in bits: {}", segmented_message.total_size());
        }
        Err(e) => println!("Error: {}", e),
    }
}
```

## API

The main struct is `SegmentedMessage`. Here are some of its key methods:

- #### ```new(message: &str, encoding: Encoding, smart_encoding: bool) -> Result<SegmentedMessage, String>```
  Initializes struct with a given message
- #### ```get_encoding_name() -> Encoding```
  Returns the detected encoding
- #### ```total_size() -> u16```
  Returns the total message size in bits
- #### ```message_size() -> u16```
  Returns the message size in bits (minus UDH)
- #### ```segments_count() -> usize```
  Returns count of segments
- #### ```get_non_gsm_characters() -> HashSet<String>```
  Returns vector of all UCS2 characters detected in the message

The other structs are exposed and can be used for their specific operations.

A simple CLI tool is included and can be accessed with `cargo run -- "Message`. It will print out the statistics for a
given string.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
