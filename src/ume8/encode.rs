use crate::ume8::{MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_END, MASK_SEQ_START, MASK_SEQ_START_DATA};

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    pub iter: I,
    current_sequence: Option<EncodeSequenceUnchecked>,
}

impl <I> EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            current_sequence: None,
        }
    }
}

impl <I> Iterator for EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sequence.is_none() {
            self.current_sequence = Some(EncodeSequenceUnchecked::new(self.iter.next()?));
        }

        self.current_sequence
            .as_mut()
            .unwrap()
            .next()
            .or_else(|| {
                self.current_sequence = None;
                self.next()
            })
    }
}

#[derive(Clone)]
pub struct EncodeSequenceUnchecked {
    data: u32,
    current: u8,
}

impl EncodeSequenceUnchecked {
    pub fn new(data: u32) -> Self {
        Self {
            data,
            current: 0,
        }
    }
}

impl Iterator for EncodeSequenceUnchecked {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        // 1 byte
        if self.data & 0b1111_1111_1111_1111_1111_1111_1000_0000 == 0 {
            return match self.current - 1 {
                0 => Some(self.data as u8),
                _ => None,
            };
        }

        // 2 bytes
        if self.data & 0b1111_1111_1111_1111_1111_1000_0000_0000 == 0 {
            return match self.current - 1 {
                0 => Some((((self.data >> 5) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START),
                1 => Some(((self.data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END),
                _ => None,
            };
        }

        // 3 bytes
        if self.data & 0b1111_1111_1111_1111_0000_0000_0000_0000 == 0 {
            return match self.current - 1 {
                0 => Some((((self.data >> (5+5)) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START),
                1 => Some((((self.data >> 5) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ),
                2 => Some(((self.data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END),
                _ => None,
            };
        }

        // 4 bytes
        if self.data & 0b1111_1111_1110_0000_0000_0000_0000_0000 == 0 {
            return match self.current - 1 {
                0 => Some((((self.data >> (5+5+5)) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START),
                1 => Some((((self.data >> (5+5)) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ),
                2 => Some((((self.data >> 5) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ),
                3 => Some(((self.data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END),
                _ => None,
            };
        }

        panic!("trying to encode more than 21 bits of data");
    }
}