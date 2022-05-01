pub fn count_data_bits<'a, I>(raw_bytes: I) -> usize
    where I: Iterator<Item=&'a u8>
{
    let mut data_bits: usize = 0;

    for raw_byte in raw_bytes {
        data_bits += 8;

        // no data encountered as of yet
        if data_bits == 8 {
            data_bits -= raw_byte.leading_zeros() as usize;
        }
    }

    data_bits
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DataBytes<'a, I>
    where I: Iterator<Item=&'a u8>,
{
    pub iter: I,
    encountered_data: bool,
}

impl <'a, I> DataBytes<'a, I>
    where I: Iterator<Item=&'a u8>,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            encountered_data: false,
        }
    }
}

impl <'a, I> Iterator for DataBytes<'a, I>
    where I: Iterator<Item=&'a u8>,
{
    type Item = &'a u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let byte = match self.iter.next() {
            Some(x) => x,
            None => {
                if ! self.encountered_data {
                    self.encountered_data = true;

                    return Some(&0u8);
                }

                return None;
            }
        };

        if ! self.encountered_data {
            if byte == &0u8 {
                return self.next();
            }

            self.encountered_data = true;
        }

        Some(byte)
    }
}
