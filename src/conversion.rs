use std::io::Error;
use crate::{RUNE_CHAR_INDEX, RUNE_LEN};

/// Convert a GeoHash to an int with precision `precision`.<br>
/// The int range is based on the number of possible combinations of characters from the GeoHash rune.
pub fn geohash_to_int(geohash: String) -> Result<i32, Error> {
    /* Since there are no optional arguments in Rust right now, use 0 to ignore precision */

    let mut output: usize = 0;

    for (i, char) in geohash.chars().into_iter().enumerate() {
        output += RUNE_CHAR_INDEX[&char] * RUNE_LEN.pow(i as u32);
    }

    Ok(output as i32)
}