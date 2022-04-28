use ume8::decode::DecodeUnchecked;

mod ume8;

fn main() {
    let bytes: [u8; 4] = [
        // a
        0b01100001,
        // ö
        0b11000111,
        0b10110110,
        // u
        0b01110101,
    ];

    let mut decoder = DecodeUnchecked {
        iter: bytes.iter(),
    };
    dbg!(decoder.next());
    dbg!(decoder.next());
    dbg!(decoder.next());
    dbg!(decoder.next());
    dbg!(decoder.next());
}
