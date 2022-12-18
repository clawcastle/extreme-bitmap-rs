pub trait ExtendBits {
    fn extend_at_least_n_bits(&mut self, n_bits: usize);
}

// Can be optimized slightly by taking cursor position into account, but the few potential excess bytes allocated can be shaved off in the end of the procedure.
impl ExtendBits for Vec<u8> {
    fn extend_at_least_n_bits(&mut self, n_bits: usize) {
        let n_bytes = (n_bits / 8) + 1;

        self.extend_from_slice(&vec![0; n_bytes]);
    }
}
