use std::collections::HashSet;

pub struct ExtremeBitmap {}

impl ExtremeBitmap {
    pub fn new() -> Self {
        ExtremeBitmap { }
    }

    pub fn extreme_bitmap(&mut self, input: &Vec<u8>) -> Vec<u8> {
        let mut symbol_counts: [usize; 256] = [0; 256];
        let mut symbol_ranges: [usize; 256] = [0; 256];

        // Get count of each unique symbol in input.
        for n in input {
            symbol_counts[*n as usize] += 1;
        }

        // Sum symbol counts, representing indices where they are supposed to be in the sorted vector.
        for i in 0..symbol_counts.len() {
            if i > 0 {
                symbol_ranges[i] += symbol_ranges[i - 1];
            }
            symbol_ranges[i] += symbol_counts[i];
        }

        // Hash-set containing indices of all elements that are already in the correct place.
        let mut indices_to_skip: HashSet<usize> = HashSet::with_capacity((input.len() / 8) + 1);
        // Initial bitmap where each bit represents the position of an element in the input vector.
        let mut initial_bitmap = vec![0; (input.len() / 8) + 1];

        // If an element in the input vector is out of place, flip the bit corresponding to its position in the bitmap.
        // Else, add its index to the indices_to_skip hash-set
        for (i, n) in input.iter().enumerate() {
            let range_idx = *n as usize;
            let current_range_end = symbol_ranges[range_idx];
            let previous_range_end = if range_idx > 0 { symbol_ranges[range_idx - 1] } else { 0 };

            let symbol_is_in_wrong_range = i >= current_range_end || i < previous_range_end;

            if symbol_is_in_wrong_range {
                let byte_idx = i / 8;
                let bit_idx: u8 = (i % 8) as u8;
    
                initial_bitmap[byte_idx] |= 1 << bit_idx;
            } else {
                indices_to_skip.insert(i);
            }
        }

        initial_bitmap
    }
}

fn main() {
    let mut x = ExtremeBitmap::new();

    let res = x.extreme_bitmap(&vec![1,2,3]);

    println!("{:?}", res);
}
