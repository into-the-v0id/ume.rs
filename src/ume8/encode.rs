use crate::ume8::{
    MASK_SEQ, MASK_SEQ_CONT_DATA, MASK_SEQ_END, MASK_SEQ_START, MASK_SEQ_START_DATA,
};

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncodeUnchecked<I>
where
    I: Iterator<Item = u32>,
{
    pub iter: I,
    buffer: [u8; 4],
    next_index: u8,
}

impl<I> EncodeUnchecked<I>
where
    I: Iterator<Item = u32>,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            buffer: [0, 0, 0, 0],
            next_index: 4,
        }
    }

    fn set_data(&mut self, data: u32) {
        // 1 byte
        if data & 0b1111_1111_1111_1111_1111_1111_1000_0000 == 0 {
            self.buffer = [0, 0, 0, data as u8];
            self.next_index = 3;

            return;
        }

        // 2 bytes
        if data & 0b1111_1111_1111_1111_1111_1000_0000_0000 == 0 {
            self.buffer = [
                0,
                0,
                (((data >> 5) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START,
                ((data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END,
            ];
            self.next_index = 2;

            return;
        }

        // 3 bytes
        if data & 0b1111_1111_1111_1111_0000_0000_0000_0000 == 0 {
            self.buffer = [
                0,
                (((data >> (5 + 5)) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START,
                (((data >> 5) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ,
                ((data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END,
            ];
            self.next_index = 1;

            return;
        }

        // 4 bytes
        if data & 0b1111_1111_1110_0000_0000_0000_0000_0000 == 0 {
            self.buffer = [
                (((data >> (5 + 5 + 5)) as u8) & MASK_SEQ_START_DATA) | MASK_SEQ | MASK_SEQ_START,
                (((data >> (5 + 5)) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ,
                (((data >> 5) as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ,
                ((data as u8) & MASK_SEQ_CONT_DATA) | MASK_SEQ | MASK_SEQ_END,
            ];
            self.next_index = 0;

            return;
        }

        panic!("trying to encode more than 21 bits of data");
    }
}

impl<I> Iterator for EncodeUnchecked<I>
where
    I: Iterator<Item = u32>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // end of current sequence
        if self.next_index >= 4 {
            let next_data = self.iter.next()?;

            // 1 byte
            if next_data & 0b1111_1111_1111_1111_1111_1111_1000_0000 == 0 {
                return Some(next_data as u8);
            }

            self.set_data(next_data);
        }

        let next_byte = self.buffer[self.next_index as usize];
        self.next_index += 1;

        Some(next_byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_sequences() {
        for (decoded, encoded) in super::super::tests::data() {
            let encoder = EncodeUnchecked::new(decoded.clone().into_iter());
            let encoder_data = encoder.collect::<Vec<u8>>();

            assert_eq!(encoder_data, encoded);
        }
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_encode_five_byte_sequence() {
        let data: [u32; 1] = [128512375];
        let encoder = EncodeUnchecked::new(data.clone().into_iter());
        encoder.collect::<Vec<u8>>();
    }
}
