use crate::ume8::MASK_SEQ_START;

pub fn count_sequences_unchecked<I: Iterator<Item=u8>>(iter: I) -> usize {
    iter
        .filter(|byte| byte & MASK_SEQ_START != 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_mixed_sequences() {
        let actual_encoded_data: Vec<u8> = vec![
            97,
            239, 170,
            98,
            204, 140, 189,
            99,
            195, 157, 144, 160,
            100,
        ];

        let sequence_count = count_sequences_unchecked(
            actual_encoded_data.clone().into_iter()
        );

        assert_eq!(sequence_count, 7);
    }

    #[test]
    fn test_count_ascii_sequences() {
        let actual_encoded_data: Vec<u8> = vec![
            97,
            98,
            99,
            100,
        ];

        let sequence_count = count_sequences_unchecked(
            actual_encoded_data.clone().into_iter()
        );

        assert_eq!(sequence_count, 4);
    }

    #[test]
    fn test_count_multibyte_sequences() {
        let actual_encoded_data: Vec<u8> = vec![
            239, 170,
            204, 140, 189,
            195, 157, 144, 160,
        ];

        let sequence_count = count_sequences_unchecked(
            actual_encoded_data.clone().into_iter()
        );

        assert_eq!(sequence_count, 3);
    }
}
