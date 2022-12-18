use itertools::Itertools;

#[derive(Clone)]
pub struct ExtremeBitmapHeader {
    symbol_counts_size_flag: SymbolCountByteSizeFlag,
    symbol_counts: Vec<u8>,
    symbols_sorted_by_amount_out_of_place: Vec<u8>,
}

impl ExtremeBitmapHeader {
    pub fn new(symbol_counts: &[usize], symbols_sorted_by_amount_out_of_place: &[usize]) -> Self {
        let max_count = symbol_counts
            .iter()
            .max()
            .expect("Symbols counts cannot be empty");

        let symbol_counts_size_flag = SymbolCountByteSizeFlag::from_max_count(*max_count).unwrap();

        let symbol_counts = symbol_counts
            .iter()
            .flat_map(|count| symbol_counts_size_flag.convert_count_to_bytes(*count))
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

impl From<ExtremeBitmapHeader> for Vec<u8> {
    fn from(val: ExtremeBitmapHeader) -> Self {
        [
            vec![val.symbol_counts_size_flag.as_byte_flag()],
            val.symbol_counts,
            val.symbols_sorted_by_amount_out_of_place,
        ]
        .concat()
    }
}

#[derive(Clone, Copy)]
pub enum SymbolCountByteSizeFlag {
    One,
    Two,
    Three,
    Four,
}

impl SymbolCountByteSizeFlag {
    pub fn from_max_count(max_count: usize) -> Result<Self, String> {
        if max_count <= u8::MAX as usize {
            Ok(Self::One)
        } else if max_count <= u16::MAX as usize {
            Ok(Self::Two)
        } else if max_count <= (u16::MAX as usize) * (u8::MAX as usize) {
            Ok(Self::Three)
        } else if max_count <= u32::MAX as usize {
            Ok(Self::Four)
        } else {
            Err("Count of single symbol exceeded what fits in a u32, which is not supported at the moment.".to_string())
        }
    }

    pub fn as_byte_flag(&self) -> u8 {
        match self {
            SymbolCountByteSizeFlag::One => 1,
            SymbolCountByteSizeFlag::Two => 2,
            SymbolCountByteSizeFlag::Three => 4,
            SymbolCountByteSizeFlag::Four => 8,
        }
    }

    pub fn convert_count_to_bytes(&self, count: usize) -> Vec<u8> {
        match self {
            SymbolCountByteSizeFlag::One => (count as u8).to_ne_bytes().to_vec(),
            SymbolCountByteSizeFlag::Two => (count as u16).to_ne_bytes().to_vec(),
            SymbolCountByteSizeFlag::Three => (count as u32).to_ne_bytes()[..3].to_vec(),
            SymbolCountByteSizeFlag::Four => (count as u32).to_ne_bytes().to_vec(),
        }
    }
}
