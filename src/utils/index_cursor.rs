#[derive(Default, Debug)]
pub struct IndexCursor {
    pub bit_index: u8,
    pub byte_index: usize,
    internal_index: usize,
}

impl IndexCursor {
    pub fn advance(&mut self) {
        self.internal_index += 1;
        self.byte_index = self.internal_index / 8;
        self.bit_index = (self.internal_index & 7) as u8; // Same as modulo 8, but faster
    }

    pub fn bit_count(&self) -> usize {
        self.byte_index * 8 + (self.bit_index as usize)
    }

    pub fn byte_count(&self) -> usize {
        self.byte_index + 1 + if self.bit_index > 0 { 1 } else { 0 }
    }
}
