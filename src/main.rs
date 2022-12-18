#![feature(let_chains)]

use crate::{extreme_bitmap::ExtremeBitmap, serialize::ExtremeBitmapSerializer};

mod extreme_bitmap;
mod header;
mod serialize;
mod utils;
fn main() {
    let input = vec![2, 1, 3];
    let data = ExtremeBitmap::serialize_from_slice(&input);

    println!("{:?}", data);
}
