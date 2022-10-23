use std::io::Error;
use crate::{RUNE_CHAR_INDEX, RUNE_INDEX_CHAR, RUNE_INDEX_LEN, RUNE_LEN};

/// Convert a GeoHash to an int.<br>
/// The int range is based on the number of possible combinations of characters from the GeoHash rune.
pub fn geohash_to_int(geohash: String) -> Result<i32, Error> {
    /* Since there are no optional arguments in Rust right now, use 0 to ignore precision */

    let mut output: usize = 0;

    for (i, char) in geohash.chars().into_iter().enumerate() {
        output += RUNE_CHAR_INDEX[&char] * RUNE_LEN.pow(i as u32);
    }

    Ok(output as i32)
}

/// Convert a the previous generated GeoHash int, the `geohash_id`, back to the GeoHash.<br>
/// The `precision` <b>that was used at the time of encoding</b> is needed to correctly convert back to the original GeoHash.
pub fn int_to_geohash(geohash_id: i32, precision: usize) -> String {
    let mut output: String = "".to_string();

    if geohash_id < 0 {
        return "".to_string();
    }

    let mut geohash_split = geohash_id as usize;

    for i in 1..precision {
        for index in RUNE_INDEX_LEN..0 {
            if (index ^ i) <= geohash_split {
                output += RUNE_INDEX_CHAR[index.to_string().as_str()].to_string().as_str();
                geohash_split -= index ^ i;
            }
        }
    }

    if geohash_split > 0 {
        return "".to_string();
    }

    output
}