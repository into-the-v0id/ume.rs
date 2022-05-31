use crate::ume8::decode::DecodeUnchecked;
use crate::ume8::encode::EncodeUnchecked;
use crate::ume8::str::Str;
use crate::ume8::string::String as Ume8String;

mod ume8;

fn main() {
    string();
}

#[allow(dead_code)]
fn string() {
    let str = Ume8String::from("aöu😀");
    println!("{}", str);

    let str: &Str = &str;
    println!("{}", str);
}

#[allow(dead_code)]
fn encode_decode() {
    let chars = vec!['a', 'ö', 'u', '😀'];
    dbg!(chars.clone());

    let raw_chars = chars.into_iter()
        .map(|c| c as u32)
        .collect::<Vec<u32>>();
    dbg!(
        raw_chars.iter()
            .map(|b| format!("{:#034b}", b))
            .collect::<Vec<String>>()
    );

    let encoded_chars = EncodeUnchecked::new(raw_chars.into_iter())
        .collect::<Vec<u8>>();
    dbg!(
        encoded_chars.iter()
            .map(|b| format!("{:#010b}", b))
            .collect::<Vec<String>>()
    );

    let decoded_raw_chars = DecodeUnchecked::new(encoded_chars.into_iter())
        .collect::<Vec<u32>>();
    dbg!(
        decoded_raw_chars.iter()
            .map(|b| format!("{:#034b}", b))
            .collect::<Vec<String>>()
    );

    let decoded_chars = decoded_raw_chars.into_iter()
        .map(|c| char::from_u32(c).unwrap())
        .collect::<Vec<char>>();
    dbg!(decoded_chars);
}
