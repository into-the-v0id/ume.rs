#![allow(dead_code)]

use crate::ume8::{MASK_SEQ, MASK_SEQ_START};

pub fn count_sequences_unchecked<I: Iterator<Item = u8>>(iter: I) -> usize {
    iter.filter(|byte| byte & MASK_SEQ == 0 || byte & MASK_SEQ_START != 0)
        .count()
}

#[inline]
pub fn is_sequence_part(byte: &u8) -> bool {
    byte & MASK_SEQ != 0
}

#[inline]
pub fn is_singleton(byte: &u8) -> bool {
    byte & MASK_SEQ == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_sequences() {
        for (decoded, encoded) in super::super::tests::data() {
            let sequence_count = count_sequences_unchecked(encoded.clone().into_iter());

            assert_eq!(sequence_count, decoded.len());
        }
    }

    #[test]
    fn test_is_sequence_part() {
        assert_eq!(is_sequence_part(&0b10000000), true);
        assert_eq!(is_sequence_part(&0b11111111), true);
        assert_eq!(is_sequence_part(&0b00000000), false);
        assert_eq!(is_sequence_part(&0b01111111), false);
    }

    #[test]
    fn test_is_singleton() {
        assert_eq!(is_singleton(&0b10000000), false);
        assert_eq!(is_singleton(&0b11111111), false);
        assert_eq!(is_singleton(&0b00000000), true);
        assert_eq!(is_singleton(&0b01111111), true);
    }
}
