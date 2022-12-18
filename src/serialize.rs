use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    extreme_bitmap::ExtremeBitmap,
    header::ExtremeBitmapHeader,
    utils::{extend_vec::ExtendBits, index_cursor::IndexCursor},
};

pub trait ExtremeBitmapSerializer {
    fn serialize_from_slice(input: &[u8]) -> Vec<u8>;
}

impl ExtremeBitmapSerializer for ExtremeBitmap {
    fn serialize_from_slice(input: &[u8]) -> Vec<u8> {
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

        let mut cursor = IndexCursor::default();

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
                initial_bitmap[cursor.byte_index] |= 1 << cursor.bit_index;
                symbols_out_of_place[*n as usize] += 1;
            } else {
                indices_to_skip.insert(i);
            }

            cursor.advance();
        }

        let symbols_sorted_by_out_of_place: Vec<usize> = symbols_out_of_place
            .into_iter()
            .enumerate()
            .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
            .map(|x| x.0)
            .collect();

        for n in symbols_sorted_by_out_of_place.clone() {
            let n_symbols = symbol_counts[n];

            println!(
                "{}, {}, {}",
                input.len(),
                indices_to_skip.len(),
                symbol_counts[n]
            );

            // We will need this amount of bits to represent this range. However, the subtraction might overflow, which is why we do checked_sub
            let range_len_bits = if let Some (n) = input.len().checked_sub(indices_to_skip.len()) && let Some (nn) = n.checked_sub(symbol_counts[n]) {
                nn
            } else {0};

            if symbol_counts[n] == 0 {
                continue;
            }

            initial_bitmap.extend_at_least_n_bits(range_len_bits);

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
            (0..start_idx).for_each(|i| {
                if indices_to_skip.contains(&i) {
                    return;
                }

                if input[i] == n as u8 {
                    initial_bitmap[cursor.byte_index] |= 1 << cursor.bit_index;
                    indices_to_skip.insert(i);
                }

                cursor.advance();
            });

            //From end_idx
            (end_idx..input.len()).for_each(|i| {
                if indices_to_skip.contains(&i) {
                    return;
                }

                if input[i] == n as u8 {
                    initial_bitmap[cursor.byte_index] |= 1 << cursor.bit_index;
                    indices_to_skip.insert(i);
                }

                cursor.advance();
            });
        }

        let header = ExtremeBitmapHeader::new(&symbol_counts, &symbols_sorted_by_out_of_place);

        let header_serialized: Vec<u8> = header.into();

        [&header_serialized, &initial_bitmap[..cursor.byte_count()]].concat()
    }
}
