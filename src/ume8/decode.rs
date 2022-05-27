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
    fn test_decode_sequences_to_char() {
        for (decoded, encoded) in super::super::tests::data() {
            let decoded_chars = decoded.clone().into_iter()
                .map(|data| char::from_u32(data).unwrap())
                .collect::<Vec<char>>();

            let decoder = unsafe {
                ToCharUnchecked::new(
                    DecodeUnchecked::new(
                        encoded.clone().into_iter()
                    )
                )
            };
            let decoder_data = decoder.collect::<Vec<char>>();

            assert_eq!(decoder_data, decoded_chars);
        }
    }

    #[test]
    fn test_decode_sequences_to_char_reverse() {
        for (decoded, encoded) in super::super::tests::data() {
            let decoded_chars_reversed = decoded.iter()
                .map(|&data| char::from_u32(data).unwrap())
                .rev()
                .collect::<Vec<char>>();

            let decoder = unsafe {
                ToCharUnchecked::new(
                    DecodeUnchecked::new(
                        encoded.clone().into_iter()
                    )
                )
            };
            let decoder_data = decoder.rev().collect::<Vec<char>>();

            assert_eq!(decoder_data, decoded_chars_reversed);
        }
    }

    #[test]
    fn test_decode_sequences() {
        for (decoded, encoded) in super::super::tests::data() {
            let decoder = DecodeUnchecked::new(encoded.clone().into_iter());
            let decoder_data = decoder.collect::<Vec<u32>>();

            assert_eq!(decoder_data, decoded);
        }
    }

    #[test]
    fn test_decode_sequences_reverse() {
        for (decoded, encoded) in super::super::tests::data() {
            let decoded_reverse = decoded.iter()
                .map(|&data| data)
                .rev()
                .collect::<Vec<u32>>();

            let decoder = DecodeUnchecked::new(encoded.clone().into_iter());
            let decoder_data = decoder.rev().collect::<Vec<u32>>();

            assert_eq!(decoder_data, decoded_reverse);
        }
    }

    #[test]
    fn test_decode_size_hint() {
        for (decoded, encoded) in super::super::tests::data() {
            let decoder = DecodeUnchecked::new(encoded.clone().into_iter());
            let decoder_size_hint = decoder.size_hint();
            let decoder_size_hint_lower = decoder_size_hint.0;
            let decoder_size_hint_upper = decoder_size_hint.1.unwrap();

            assert!(decoder_size_hint_lower <= decoded.len());
            assert!(decoder_size_hint_upper >= decoded.len());
        }
    }

    #[test]
    fn test_decode_count() {
        for (decoded, encoded) in super::super::tests::data() {
            let decoder = DecodeUnchecked::new(encoded.clone().into_iter());
            let decoder_count = decoder.count();

            assert_eq!(decoder_count, decoded.len());
        }
    }
}
