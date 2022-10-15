# UME

A Rust implementation of the UME Character Encoding

## Specification

UME has no real specification yet. For now, this implementation serves as the primary definition.

Binary representation of sequences with 1-4 bytes (data displayed as "x"):
| Byte 1   | Byte 2   | Byte 3   | Byte 4   |
| -------- | -------- | -------- | -------- |
| 0xxxxxxx |          |          |          |
| 11xxxxxx | 101xxxxx |          |          |
| 11xxxxxx | 100xxxxx | 101xxxxx |          |
| 11xxxxxx | 100xxxxx | 100xxxxx | 101xxxxx |

Byte order of data: Big-Endian  
Bit order of data: most significant bit first (MSB 0)

Example:
| Char   | Unicode code point | Binary data       | UME encoded       |
| ------ | ------------------ | ----------------- | ----------------- |
| a      | U+0061             | 01100001          | 01100001          |
| ӕ      | U+04D5             | 00000100 11010101 | 11100110 10110101 |

## Installation

This crate is not available on crates.io. To use it you will have to link the source directly:
```toml
[dependencies]
ume = { git = "https://github.com/into-the-v0id/ume.rs" }
```

## Usage

Strings:
```rust
use ume::ume8::string::Ume8String;
use ume::ume8::str::Ume8Str;

pub fn main() {
    let string: Ume8String = Ume8String::from("aöӕธ💻");
    let str: &Ume8Str = &string;

    assert_eq!(str.chars().count(), 5);
    assert_eq!(str.contains(&Ume8String::from('ӕ')), true);
}
```

Streams:
```rust
use ume::ume8::decode::DecodeUnchecked;
use ume::ume8::encode::EncodeUnchecked;

pub fn main() {
    let data = vec![
        'a' as u32,
        'ö' as u32,
        'ӕ' as u32,
        'ธ' as u32,
        '💻' as u32,
    ];

    let encoded_data = EncodeUnchecked::new(data.iter().cloned())
        .collect::<Vec<u8>>();

    let decoded_data = DecodeUnchecked::new(encoded_data.iter().cloned())
        .collect::<Vec<u32>>();

    assert_eq!(decoded_data, data);
}
```

## Limitations

In theory, a single sequence can contain an unlimited amount of bytes. For performance reasons, this implementation limits the size of a single sequence to 4 bytes and thus 21 bits of data.

## License

Copyright (C) Oliver Amann

This project is licensed under the MIT License (MIT) or the Apache License Version 2.0 (Apache-2.0). Please see [LICENSE-MIT](./LICENSE-MIT) and [LICENSE-APACHE](./LICENSE-APACHE) for more information.
