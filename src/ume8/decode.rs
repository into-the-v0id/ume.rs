use std::marker::PhantomData;
use std::ops::{BitOr, Shl};

const MASK_SEQ: u8 = 0b10000000;
const MASK_SEQ_START_DATA: u8 = 0b00111111;
const MASK_SEQ_CONT_DATA: u8 = 0b00011111;
const MASK_SEQ_END: u8 = 0b00100000;

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DecodeUnchecked<'a, D, I>
    where
        D: From<u8> + Shl<u8, Output=D> + BitOr<D, Output=D>,
        I: Iterator<Item=&'a u8>
{
    pub iter: I,
    data: PhantomData<D>,
}

impl <'a, D, I> DecodeUnchecked<'a, D, I>
    where
        D: From<u8> + Shl<u8, Output=D> + BitOr<D, Output=D>,
        I: Iterator<Item=&'a u8>
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            data: PhantomData,
        }
    }
}

impl <'a, D, I> Iterator for DecodeUnchecked<'a, D, I>
    where
        D: From<u8> + Shl<u8, Output=D> + BitOr<D, Output=D>,
        I: Iterator<Item=&'a u8>
{
    type Item = D;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let first_byte = *self.iter.next()?;
        if first_byte & MASK_SEQ == 0 {
            return Some(D::from(first_byte));
        }

        let mut data = D::from(first_byte & MASK_SEQ_START_DATA);

        loop {
            let next_byte = *self.iter.next().unwrap();

            data = (data << 5) | D::from(next_byte & MASK_SEQ_CONT_DATA);

            if next_byte & MASK_SEQ_END != 0 {
                break;
            }
        }

        Some(data)
    }
}
