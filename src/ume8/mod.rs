mod decode;
mod encode;
mod str;
mod string;
mod util;

pub use self::decode::*;
pub use self::encode::*;
pub use self::str::*;
pub use self::string::*;
pub use self::util::*;

const MASK_SEQ: u8 = 0b10000000;
const MASK_SEQ_START: u8 = 0b01000000;
const MASK_SEQ_END: u8 = 0b00100000;
const MASK_SEQ_START_DATA: u8 = 0b00111111;
const MASK_SEQ_CONT_DATA: u8 = 0b00011111;

#[cfg(test)]
pub(crate) mod tests {
    pub(crate) fn data() -> Vec<(Vec<u32>, Vec<u8>)> {
        vec![
            (
                vec![97, 1514, 33, 12701, 0, 128512, 100],
                vec![97, 239, 170, 33, 204, 140, 189, 0, 195, 157, 144, 160, 100],
            ),
            (vec![97, 33, 0, 100], vec![97, 33, 0, 100]),
            (
                vec![1514, 12701, 128512],
                vec![239, 170, 204, 140, 189, 195, 157, 144, 160],
            ),
            (vec![97], vec![97]),
            (vec![1514], vec![239, 170]),
            (vec![12701], vec![204, 140, 189]),
            (vec![128512], vec![195, 157, 144, 160]),
            (vec![0], vec![0]),
            (vec![], vec![]),
        ]
    }
}
