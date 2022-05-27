use crate::ume8::{MASK_SEQ, MASK_SEQ_START};

pub fn count_sequences_unchecked<I: Iterator<Item=u8>>(iter: I) -> usize {
    iter
        .filter(|byte| byte & MASK_SEQ == 0 || byte & MASK_SEQ_START != 0)
        .count()
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
}
