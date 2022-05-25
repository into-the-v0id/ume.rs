pub mod encode;
pub mod decode;
pub mod string;

const MASK_SEQ: u8 = 0b10000000;
const MASK_SEQ_START: u8 = 0b01000000;
const MASK_SEQ_END: u8 = 0b00100000;
const MASK_SEQ_START_DATA: u8 = 0b00111111;
const MASK_SEQ_CONT_DATA: u8 = 0b00011111;
