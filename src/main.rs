use itertools::Itertools;
use std::collections::HashSet;

pub struct ExtremeBitmap {}

impl ExtremeBitmap {
    pub fn new() -> Self {
        ExtremeBitmap {}
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

        let mut symbols_out_of_place: [usize; 256] = [0; 256];

        // If an element in the input vector is out of place, flip the bit corresponding to its position in the bitmap.
        // Else, add its index to the indices_to_skip hash-set
        for (i, n) in input.iter().enumerate() {
            let range_idx = *n as usize;
            let current_range_end = symbol_ranges[range_idx];
            let previous_range_end = if range_idx > 0 {
                symbol_ranges[range_idx - 1]
            } else {
                0
            };

            let symbol_is_in_wrong_range = i >= current_range_end || i < previous_range_end;

            if symbol_is_in_wrong_range {
                let byte_idx = i / 8;
                let bit_idx: u8 = (i % 8) as u8;

                initial_bitmap[byte_idx] |= 1 << bit_idx;
            } else {
                indices_to_skip.insert(i);
                symbols_out_of_place[i] += 1;
            }
        }

        let symbols_sorted_by_out_of_place: Vec<usize> = symbols_out_of_place
            .into_iter()
            .enumerate()
            .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
            .map(|x| x.0)
            .collect();

        for n in symbols_sorted_by_out_of_place {
            let n_symbols = symbol_counts[n];

            let start_idx = if n > 0 { symbol_ranges[n - 1] } else { 0 };
            let end_idx = if n < 255 {
                symbol_ranges[n]
            } else {
                input.len()
            };

            if n_symbols == 0 {
                continue;
            }

            // Up to start_idx
            for i in 0..start_idx {
                if indices_to_skip.contains(&i) {
                    continue;
                }

                if input[i] == n as u8 {
                    // TODO: element is n and out of place, add 1 to bitmap
                    indices_to_skip.insert(i);
                } else {
                    // TODO: element is not n, add 0 to bitmap
                }
            }

            //From end_idx
            for i in end_idx..input.len() {
                if indices_to_skip.contains(&i) {
                    continue;
                }

                if input[i] == n as u8 {
                    // TODO: element is n and out of place, add 1 to bitmap
                    indices_to_skip.insert(i);
                } else {
                    // TODO: element is not n, add 0 to bitmap
                }
            }
        }

        initial_bitmap
    }
}

impl Default for ExtremeBitmap {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut x = ExtremeBitmap::default();

    let res = x.extreme_bitmap(&vec![1, 2, 3]);

    println!("{:?}", res);
}
