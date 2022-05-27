use std::iter::FusedIterator;
use crate::ume8::{MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_END, MASK_SEQ_START, MASK_SEQ_START_DATA};
use crate::ume8::util;

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DecodeUnchecked<I>
    where I: DoubleEndedIterator<Item=u8>
{
    pub iter: I,
}

impl <I> DecodeUnchecked<I>
    where I: DoubleEndedIterator<Item=u8>
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter
        }
    }
}

impl <I> Iterator for DecodeUnchecked<I>
    where I: DoubleEndedIterator<Item=u8>
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let first_byte = self.iter.next()?;
        if first_byte & MASK_SEQ == 0 {
            return Some(first_byte as u32);
        }

        let mut data = (first_byte & MASK_SEQ_START_DATA) as u32;

        loop {
            let next_byte = self.iter.next().unwrap();

            data = (data << 5) | ((next_byte & MASK_SEQ_CONT_DATA) as u32);

            if next_byte & MASK_SEQ_END != 0 {
                break;
            }
        }

        Some(data)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        ((lower + 3) / 4, upper)
    }

    fn count(self) -> usize {
        util::count_sequences_unchecked(self.iter)
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl <I> DoubleEndedIterator for DecodeUnchecked<I>
    where I: DoubleEndedIterator<Item=u8>,
{
    // TODO: check again
    fn next_back(&mut self) -> Option<Self::Item> {
        let last_byte = self.iter.next_back()?;
        if last_byte & MASK_SEQ == 0 {
            return Some(last_byte as u32);
        }

        let mut data = (last_byte & MASK_SEQ_CONT_DATA) as u32;
        let mut bit_count: u8 = 5;

        loop {
            let prev_byte = self.iter.next_back().unwrap();

            if prev_byte & MASK_SEQ_START != 0 {
                data = data | ((prev_byte & MASK_SEQ_START_DATA) as u32) << bit_count;

                break;
            }

            data = data | ((prev_byte & MASK_SEQ_CONT_DATA) as u32) << bit_count;
            bit_count += 5;
        }

        Some(data)
    }
}

impl <I> FusedIterator for DecodeUnchecked<I>
    where I: DoubleEndedIterator<Item=u8> + FusedIterator<Item=u8>,
{}

#[derive(Clone)]
pub struct ToCharUnchecked<Iter>
    where Iter: Iterator<Item=u32>,
{
    iter: Iter,
}

impl <Iter> ToCharUnchecked<Iter>
    where Iter: Iterator<Item=u32>,
{
    #[inline]
    pub unsafe fn new(iter: Iter) -> Self {
        Self {
            iter
        }
    }
}

impl <Iter> Iterator for ToCharUnchecked<Iter>
    where Iter: Iterator<Item=u32>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|data| unsafe { char::from_u32_unchecked(data) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize where Self: Sized {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item> where Self: Sized {
        self.iter.last()
            .map(|data| unsafe { char::from_u32_unchecked(data) })
    }
}

impl<Iter> DoubleEndedIterator for ToCharUnchecked<Iter>
    where Iter: DoubleEndedIterator<Item=u32>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
            .map(|data| unsafe { char::from_u32_unchecked(data) })
    }
}

impl<Iter> ExactSizeIterator for ToCharUnchecked<Iter>
    where Iter: ExactSizeIterator<Item=u32>,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl <Iter> FusedIterator for ToCharUnchecked<Iter>
    where Iter: FusedIterator<Item=u32>,
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_to_char() {
        let actual_decoded_data = vec![
            'a',
            'ö',
            'u',
            '😀',
        ];
        let actual_encoded_data: Vec<u8> = vec![
            97,
            199, 182,
            117,
            195, 157, 144, 160,
        ];

        let decoder = unsafe {
            ToCharUnchecked::new(
                DecodeUnchecked::new(
                    actual_encoded_data.into_iter()
                )
            )
        };
        let decoded_data = decoder.collect::<Vec<char>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_to_char_reverse() {
        let actual_decoded_data = vec![
            '😀',
            'u',
            'ö',
            'a',
        ];
        let actual_encoded_data: Vec<u8> = vec![
            97,
            199, 182,
            117,
            195, 157, 144, 160,
        ];

        let decoder = unsafe {
            ToCharUnchecked::new(
                DecodeUnchecked::new(
                    actual_encoded_data.into_iter()
                )
            )
        };
        let decoded_data = decoder.rev().collect::<Vec<char>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_mixed_sequences() {
        let actual_decoded_data: Vec<u32> = vec![
            97,
            246,
            117,
            128512,
        ];
        let actual_encoded_data: Vec<u8> = vec![
            97,
            199, 182,
            117,
            195, 157, 144, 160,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoded_data = decoder.collect::<Vec<u32>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_mixed_sequences_reverse() {
        let actual_decoded_data: Vec<u32> = vec![
            128512,
            117,
            246,
            97,
        ];
        let actual_encoded_data: Vec<u8> = vec![
            97,
            199, 182,
            117,
            195, 157, 144, 160,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoded_data = decoder.rev().collect::<Vec<u32>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_ascii_sequences() {
        let actual_decoded_data: Vec<u32> = vec![
            97,
            98,
            99,
            100,
        ];
        let actual_encoded_data: Vec<u8> = vec![
            97,
            98,
            99,
            100,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoded_data = decoder.collect::<Vec<u32>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_ascii_sequences_reverse() {
        let actual_decoded_data: Vec<u32> = vec![
            100,
            99,
            98,
            97,
        ];
        let actual_encoded_data: Vec<u8> = vec![
            97,
            98,
            99,
            100,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoded_data = decoder.rev().collect::<Vec<u32>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_multibyte_sequences() {
        let actual_decoded_data: Vec<u32> = vec![
            1514,
            12701,
            128512,
        ];
        let actual_encoded_data: Vec<u8> = vec![
            239, 170,
            204, 140, 189,
            195, 157, 144, 160,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoded_data = decoder.collect::<Vec<u32>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_multibyte_sequences_reverse() {
        let actual_decoded_data: Vec<u32> = vec![
            128512,
            12701,
            1514,
        ];
        let actual_encoded_data: Vec<u8> = vec![
            239, 170,
            204, 140, 189,
            195, 157, 144, 160,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoded_data = decoder.rev().collect::<Vec<u32>>();

        assert_eq!(decoded_data, actual_decoded_data);
    }

    #[test]
    fn test_decode_size_hint() {
        let actual_encoded_data: Vec<u8> = vec![
            97,
            239, 170,
            98,
            204, 140, 189,
            99,
            195, 157, 144, 160,
            100,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoder_size_hint = decoder.size_hint();

        assert_eq!(
            decoder_size_hint,
            (4, Some(13))
        );
    }

    #[test]
    fn test_decode_count() {
        let actual_encoded_data: Vec<u8> = vec![
            97,
            239, 170,
            98,
            204, 140, 189,
            99,
            195, 157, 144, 160,
            100,
        ];

        let decoder = DecodeUnchecked::new(actual_encoded_data.into_iter());
        let decoder_count = decoder.count();

        assert_eq!(decoder_count, 7);
    }
}
