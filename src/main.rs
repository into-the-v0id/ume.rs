use crate::ume8::decode::DecodeUnchecked;
use crate::ume8::encode::EncodeUnchecked;

mod ume8;

fn main() {
    let chars = vec!['a', 'ö', 'u', '😀'];
    dbg!(chars.clone());

    let raw_chars = chars.clone().into_iter()
        .map(|c| c as u32)
        .collect::<Vec<u32>>();
    dbg!(
        raw_chars.iter()
            .map(|b| format!("{:#034b}", b))
            .collect::<Vec<String>>()
    );

    let encoded_chars = EncodeUnchecked::new(raw_chars.clone().into_iter())
        .collect::<Vec<u8>>();
    dbg!(
        encoded_chars.iter()
            .map(|b| format!("{:#010b}", b))
            .collect::<Vec<String>>()
    );

    let decoded_raw_chars = DecodeUnchecked::new(encoded_chars.clone().into_iter())
        .collect::<Vec<u32>>();
    dbg!(
        decoded_raw_chars.iter()
            .map(|b| format!("{:#034b}", b))
            .collect::<Vec<String>>()
    );

    let decoded_chars = decoded_raw_chars.clone().into_iter()
        .map(|c| char::from_u32(c).unwrap())
        .collect::<Vec<char>>();
    dbg!(decoded_chars);
}
