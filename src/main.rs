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

        let mut initial_bitmap = vec![0; (input.len() / 8) + 1];

        for (i, n) in input.iter().enumerate() {
            let range_idx = *n as usize;
            let current_range_end = symbol_ranges[range_idx];
            let previous_range_end = if range_idx > 0 { symbol_ranges[range_idx - 1] } else { 0 };

            let symbol_is_in_wrong_range = i >= current_range_end || i < previous_range_end;

            if symbol_is_in_wrong_range {
                let byte_idx = i / 8;
                let bit_idx: u8 = (i % 8) as u8;
    
                initial_bitmap[byte_idx] |= 1 << bit_idx;
            }
        }

        initial_bitmap
    }
}

fn main() {
    let mut x = ExtremeBitmap::new();

    let res = x.extreme_bitmap(&vec![2,3,1]);

    println!("{:?}", res);
}
