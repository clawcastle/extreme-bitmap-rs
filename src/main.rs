use itertools::Itertools;
use std::collections::HashSet;

pub struct ExtremeBitmap {
    data: Vec<u8>,
}

impl ExtremeBitmap {
    pub fn from_unsorted_symbols(input: &Vec<u8>) -> Self {
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

            // We will need this amount of bits to represent this range.
            let range_len_bits = input.len() - indices_to_skip.len() - symbol_counts[n];

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
                    // TODO: element is n and out of place, add 1 to bitmap
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
                    // TODO: element is n and out of place, add 1 to bitmap
                    initial_bitmap[cursor.byte_index] |= 1 << cursor.bit_index;
                    indices_to_skip.insert(i);
                }

                cursor.advance();
            });
        }

        let header = ExtremeBitmapHeader::new(&symbol_counts, &symbols_sorted_by_out_of_place);

        ExtremeBitmap {
            data: [header.into(), initial_bitmap].concat(),
        }
    }
}

#[derive(Default, Debug)]
struct IndexCursor {
    bit_index: u8,
    byte_index: usize,
    internal_index: usize,
}

#[derive(Clone)]
struct ExtremeBitmapHeader {
    symbol_counts_size_flag: SymbolCountSizeFlag,
    symbol_counts: Vec<u8>,
    symbols_sorted_by_amount_out_of_place: Vec<u8>,
}

impl ExtremeBitmapHeader {
    fn new(symbol_counts: &[usize], symbols_sorted_by_amount_out_of_place: &[usize]) -> Self {
        let max_count = symbol_counts
            .iter()
            .max()
            .expect("Symbols counts cannot be empty");

        let symbol_counts_size_flag = SymbolCountSizeFlag::from_max_count(*max_count);

        let symbol_counts = symbol_counts
            .iter()
            .flat_map(|count| count.to_ne_bytes())
            .collect_vec();
        let symbols_sorted_by_amount_out_of_place = symbols_sorted_by_amount_out_of_place
            .iter()
            .map(|n| *n as u8)
            .collect_vec();

        Self {
            symbol_counts_size_flag,
            symbol_counts,
            symbols_sorted_by_amount_out_of_place,
        }
    }
}

impl Into<Vec<u8>> for ExtremeBitmapHeader {
    fn into(self) -> Vec<u8> {
        [
            vec![self.symbol_counts_size_flag.as_byte_flag()],
            self.symbol_counts,
            self.symbols_sorted_by_amount_out_of_place,
        ]
        .concat()
    }
}

#[derive(Clone, Copy)]
enum SymbolCountSizeFlag {
    OneByte,
    TwoByte,
    ThreeByte,
    FourByte,
    LargerThanFourByte,
}

const ONE_BYTE_MAX: usize = 255;
const TWO_BYTE_MAX: usize = 65535;
const THREE_BYTE_MAX: usize = 16777215;
const FOUR_BYTE_MAX: usize = 4294967295;

impl SymbolCountSizeFlag {
    fn from_max_count(max_count: usize) -> Self {
        return if max_count <= ONE_BYTE_MAX {
            Self::OneByte
        } else if max_count <= TWO_BYTE_MAX {
            Self::TwoByte
        } else if max_count <= THREE_BYTE_MAX {
            Self::ThreeByte
        } else if max_count <= FOUR_BYTE_MAX {
            Self::FourByte
        } else {
            Self::LargerThanFourByte
        };
    }

    fn as_byte_flag(&self) -> u8 {
        match self {
            SymbolCountSizeFlag::OneByte => 1,
            SymbolCountSizeFlag::TwoByte => 2,
            SymbolCountSizeFlag::ThreeByte => 4,
            SymbolCountSizeFlag::FourByte => 8,
            SymbolCountSizeFlag::LargerThanFourByte => 128,
        }
    }
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

trait ExtendBits {
    fn extend_at_least_n_bits(&mut self, n_bits: usize);
}

// Can be optimized slightly by taking cursor position into account, but the few potential excess bytes allocated can be shaved off in the end of the procedure.
impl ExtendBits for Vec<u8> {
    fn extend_at_least_n_bits(&mut self, n_bits: usize) {
        let n_bytes = (n_bits / 8) + 1;

        self.extend_from_slice(&vec![0; n_bytes]);
    }
}

fn main() {
    let extreme_bitmap = ExtremeBitmap::from_unsorted_symbols(&vec![3, 3, 3, 2, 2, 1]);

    println!("{:?}", extreme_bitmap.data);
}
