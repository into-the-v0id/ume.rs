use std::marker::PhantomData;
use std::ops::{BitOr, Shl};
use crate::ume8::{MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_END, MASK_SEQ_START_DATA};

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DecodeUnchecked<'a, I, D>
    where
        I: Iterator<Item=&'a u8>,
        D: From<u8> + Shl<u8, Output=D> + BitOr<D, Output=D>,
{
    pub iter: I,
    data: PhantomData<D>,
}

impl <'a, I, D> DecodeUnchecked<'a, I, D>
    where
        I: Iterator<Item=&'a u8>,
        D: From<u8> + Shl<u8, Output=D> + BitOr<D, Output=D>,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            data: PhantomData,
        }
    }
}

impl <'a, I, D> Iterator for DecodeUnchecked<'a, I, D>
    where
        I: Iterator<Item=&'a u8>,
        D: From<u8> + Shl<u8, Output=D> + BitOr<D, Output=D>,
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
