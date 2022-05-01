use std::cmp::min;
use std::vec::IntoIter;
use crate::ume8::{MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_START_DATA};
use crate::ume8::util::{count_data_bits, DataBytes};

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncodeUnchecked<'a, SeqIter, I>
    where
        SeqIter: Iterator<Item=&'a u8> + Clone,
        I: Iterator<Item=SeqIter>,
{
    pub iter: I,
    pub current_sequence: Option<EncodeSequenceUnchecked<'a, SeqIter>>,
}

impl <'a, SeqIter, I> EncodeUnchecked<'a, SeqIter, I>
    where
        SeqIter: Iterator<Item=&'a u8> + Clone,
        I: Iterator<Item=SeqIter>,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            current_sequence: None,
        }
    }
}

impl <'a, SeqIter, I> Iterator for EncodeUnchecked<'a, SeqIter, I>
    where
        SeqIter: Iterator<Item=&'a u8> + Clone,
        I: Iterator<Item=SeqIter>,
{
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sequence.is_none() {
            let sequence_iter = self.iter.next()?;
            self.current_sequence = Some(EncodeSequenceUnchecked::new(sequence_iter));
        }

        return match self.current_sequence.as_mut().unwrap().next() {
            Some(x) => Some(x),
            None => {
                self.current_sequence = None;
                self.next()
            },
        };
    }
}


#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncodeSequenceUnchecked<'a, I>
    where I: Iterator<Item=&'a u8> + Clone,
{
    pub iter: DataBytes<'a, I>,
    is_first: bool,
    data_bits: usize,
    bytes_needed: usize,
    bytes_yielded: usize,

    current_byte: u8,
    current_byte_used_bits: u8,
}

impl <'a, I> EncodeSequenceUnchecked<'a, I>
    where I: Iterator<Item=&'a u8> + Clone,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        let data_bits = count_data_bits(iter.clone());
        let bytes_needed = bytes_needed(data_bits);

        Self {
            iter: DataBytes::new(iter),
            is_first: true,
            data_bits,
            bytes_needed,
            bytes_yielded: 0,

            current_byte: 0,
            current_byte_used_bits: 8,
        }
    }

    fn next_bits(&mut self, num_bits: u8) -> Option<u8> {
        let mut bits: u8 = 0;
        let mut bits_len: u8 = 0;

        // has current byte data
        if self.current_byte_used_bits != 8 {
            let take_bits = min(num_bits, 8 - self.current_byte_used_bits);

            bits |= self.current_byte
                << self.current_byte_used_bits
                >> self.current_byte_used_bits
                >> (8 - self.current_byte_used_bits - take_bits);
            bits_len += take_bits;

            self.current_byte_used_bits += take_bits;
        }

        if bits_len == num_bits {
            return Some(bits);
        }

        let next_byte = match self.iter.next() {
            Some(x) => *x,
            None => panic!("Requested {} bits, but only {} remaining", num_bits, 8 - self.current_byte_used_bits),
        };

        let take_bits = num_bits - bits_len;
        bits = (bits << take_bits) | (next_byte >> (8 - take_bits));

        self.current_byte = next_byte;
        self.current_byte_used_bits = take_bits;

        Some(bits)
    }
}

impl <'a, I> Iterator for EncodeSequenceUnchecked<'a, I>
    where I: Iterator<Item=&'a u8> + Clone,
{
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes_yielded == self.bytes_needed {
            return None;
        }

        if self.bytes_needed == 1 {
            self.bytes_yielded += 1;

            return Some(*self.iter.next()?);
        }

        // first
        if self.is_first {
            // skip leading padding
            let mut padding = 8 - (self.data_bits % 8);
            if padding == 8 {
                padding = 0;
            }
            self.next_bits(padding as u8);

            self.is_first = false;
            self.bytes_yielded += 1;

            let take_bits = self.data_bits - ((self.bytes_needed - 1) * 5);
            dbg!(take_bits);

            return Some(self.next_bits(take_bits as u8)? | 0b11000000);
        }

        // last
        if self.bytes_yielded == self.bytes_needed - 1 {
            self.bytes_yielded += 1;

            return Some(self.next_bits(5)? | 0b10100000);
        }

        self.bytes_yielded += 1;

        return Some(self.next_bits(5)? | 0b10000000);
    }
}


pub fn bytes_needed(data_bits: usize) -> usize {
    if data_bits <= 7 {
        return 1;
    }

    let mut bytes_needed: usize = 1;
    bytes_needed += (data_bits - 6) / 5;
    if (data_bits - 6) % 5 != 0 {
        bytes_needed += 1;
    }

    bytes_needed
}
