use crate::ume8::MASK_SEQ_START;

pub fn count_sequences_unchecked<I: Iterator<Item=u8>>(iter: I) -> usize {
    iter
        .filter(|byte| byte & MASK_SEQ_START != 0)
        .count()
}
