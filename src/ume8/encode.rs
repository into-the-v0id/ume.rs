use std::array::IntoIter;
use crate::ume8::{MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_END, MASK_SEQ_START, MASK_SEQ_START_DATA};

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    pub iter: I,
    current_sequence: IntoIter<u8, 4>,
}

impl <I> EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            current_sequence: unsafe { IntoIter::new_unchecked(
                std::mem::transmute([0u8, 0u8, 0u8, 0u8]),
                0..0
            ) },
        }
    }
}

impl <I> Iterator for EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sequence.next()
            .or_else(|| {
                self.current_sequence = encode_sequence_unchecked(self.iter.next()?);
                self.next()
            })
    }
}


#[inline]
fn encode_sequence_unchecked(data: u32) -> IntoIter<u8, 4> {
    // 1 byte
    if data & 0b1111_1111_1111_1111_1111_1111_1000_0000 == 0 {
        return unsafe { IntoIter::new_unchecked(
            std::mem::transmute([
                data as u8,
                0u8,
                0u8,
                0u8,
            ]),
            0..1
        ) };
    }

    // 2 bytes
    if data & 0b1111_1111_1111_1111_1111_1000_0000_0000 == 0 {
        return unsafe { IntoIter::new_unchecked(
            std::mem::transmute([
                (((data >> 5) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START,
                ((data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END,
                0u8,
                0u8,
            ]),
            0..2
        ) };
    }

    // 3 bytes
    if data & 0b1111_1111_1111_1111_0000_0000_0000_0000 == 0 {
        return unsafe { IntoIter::new_unchecked(
            std::mem::transmute([
                (((data >> (5+5)) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START,
                (((data >> 5) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ,
                ((data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END,
                0u8,
            ]),
            0..3
        ) };
    }

    // 4 bytes
    if data & 0b1111_1111_1110_0000_0000_0000_0000_0000 == 0 {
        return unsafe { IntoIter::new_unchecked(
            std::mem::transmute([
                (((data >> (5+5+5)) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START,
                (((data >> (5+5)) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ,
                (((data >> 5) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ,
                ((data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END,
            ]),
            0..4
        ) };
    }

    panic!("trying to encode more than 21 bits of data");
}
