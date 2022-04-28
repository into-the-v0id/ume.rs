const MASK_SEQ: u8 = 0b10000000;
const MASK_SEQ_START_DATA: u8 = 0b00111111;
const MASK_SEQ_CONT_DATA: u8 = 0b00011111;
const MASK_SEQ_END: u8 = 0b00100000;

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DecodeUnchecked<'a, I: Iterator<Item=&'a u8>> {
    pub iter: I,
}

impl <'a, I: Iterator<Item=&'a u8>> Iterator for DecodeUnchecked<'a, I> {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let first_byte = *self.iter.next()?;
        if first_byte & MASK_SEQ == 0 {
            return Some(u32::from(first_byte));
        }

        let mut data = u32::from(first_byte & MASK_SEQ_START_DATA);

        loop {
            let next_byte = *self.iter.next().unwrap();

            data = (data << 5) | u32::from(next_byte & MASK_SEQ_CONT_DATA);

            if next_byte & MASK_SEQ_END != 0 {
                break;
            }
        }

        Some(data)
    }
}
