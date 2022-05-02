use std::vec::IntoIter;

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    pub iter: I,
    current_sequence: IntoIter<u8>,
}

impl <I> EncodeUnchecked<I>
    where I: Iterator<Item=u32>
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            current_sequence: Vec::with_capacity(0).into_iter(),
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
                self.current_sequence = encode_sequence_unchecked(self.iter.next()?).into_iter();
                self.next()
            })
    }
}


#[inline]
pub fn encode_sequence_unchecked(data: u32) -> Vec<u8> {
    // 1 byte
    if data & 0b11111111_11111111_11111111_10000000 == 0 {
        let mut bytes = Vec::with_capacity(1);

        bytes.push(data as u8);

        return bytes;
    }

    // 2 bytes
    if data & 0b11111111_11111111_11111000_00000000 == 0 {
        let mut bytes = Vec::with_capacity(3);

        bytes.push((((data >> 5) & 0b00111111) | 0b11000000) as u8);
        bytes.push(((data & 0b00011111) | 0b10100000) as u8);

        return bytes;
    }

    // 3 bytes
    if data & 0b11111111_11111111_00000000_00000000 == 0 {
        let mut bytes = Vec::with_capacity(3);

        bytes.push((((data >> 5+5) & 0b00111111) | 0b11000000) as u8);
        bytes.push((((data >> 5) & 0b00011111) | 0b10000000) as u8);
        bytes.push(((data & 0b00011111) | 0b10100000) as u8);

        return bytes;
    }

    // 4 bytes
    if data & 0b11111111_11100000_00000000_00000000 == 0 {
        let mut bytes = Vec::with_capacity(4);

        bytes.push((((data >> 5+5+5) & 0b00111111) | 0b11000000) as u8);
        bytes.push((((data >> 5+5) & 0b00011111) | 0b10000000) as u8);
        bytes.push((((data >> 5) & 0b00011111) | 0b10000000) as u8);
        bytes.push(((data & 0b00011111) | 0b10100000) as u8);

        return bytes;
    }

    panic!("trying to encode more than 21 bits of data");
}
