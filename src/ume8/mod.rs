pub mod encode;
pub mod decode;

const MASK_SEQ: u8 = 0b10000000;
const MASK_SEQ_START_DATA: u8 = 0b00111111;
const MASK_SEQ_CONT_DATA: u8 = 0b00011111;
const MASK_SEQ_END: u8 = 0b00100000;
