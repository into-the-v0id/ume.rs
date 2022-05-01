use ume8::decode::DecodeUnchecked;
use crate::ume8::encode::EncodeSequenceUnchecked;

mod ume8;

fn main() {
    let bytes = [
        // a
        0b01100001,
        // ö
        0b11000111,
        0b10110110,
        // u
        0b01110101,
    ];

    let mut decoder = DecodeUnchecked::<_, u32>::new(bytes.iter());
    dbg!(decoder.next());
    dbg!(decoder.next());
    dbg!(decoder.next());
    dbg!(decoder.next());
    dbg!(decoder.next());

    let sequences = [
        vec![0b00000001],
        vec![0b10000001],
        vec![0b00000000, 0b00000001],
        vec![0b00010000, 0b00000001],
        vec![0b00000000, 0b00000000, 0b00000001],
        vec![0b00000100, 0b00010000, 0b00000001],
    ];

    for sequence in sequences {
        dbg!(
            EncodeSequenceUnchecked::new(sequence.iter())
                .map(|b| format!("{:#010b}", b))
                .collect::<Vec<String>>()
        );
    }

    let chars = "aöu".chars()
        .map(|c| (c as u32).to_be_bytes())
        .map(|b| EncodeSequenceUnchecked::new(b.iter()).collect::<Vec<u8>>())
        .map(|b| b.iter().map(|b| format!("{:#010b}", b)).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    dbg!(chars);
}
