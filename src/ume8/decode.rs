use crate::ume8::{MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_END, MASK_SEQ_START_DATA};

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DecodeUnchecked<I>
    where I: Iterator<Item=u8>
{
    pub iter: I,
}

impl <I> DecodeUnchecked<I>
    where I: Iterator<Item=u8>
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter
        }
    }
}

impl <I> Iterator for DecodeUnchecked<I>
    where I: Iterator<Item=u8>
{
    type Item = u32;

    #[inline]
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
}

#[derive(Clone)]
pub struct ToCharUnchecked<Iter>
    where Iter: Iterator<Item=u32>,
{
    iter: Iter,
}

impl <Iter> ToCharUnchecked<Iter>
    where Iter: Iterator<Item=u32>,
{
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|data| unsafe { char::from_u32_unchecked(data) })
    }
}
